import { BastionSidecar } from "./sidecar";

/** Chain identifiers as serialized by the sidecar's core types. */
export type ChainName =
  | "Solana"
  | "Base"
  | "Ethereum"
  | "Polygon"
  | "Arbitrum"
  | "Celo"
  | "ZkSync"
  | "Robinhood";

/** The kind of on-chain action a plan leg executes. */
export type IntentActionType =
  | "Swap"
  | "Transfer"
  | "Bridge"
  | "Stake"
  | "Unstake"
  | "Lend"
  | "Borrow"
  | "DeployContract"
  | "CallContract"
  | { Custom: string };

/** A single on-chain action with all parameters needed for execution. */
export interface TrustIntentAction {
  action_type: IntentActionType;
  chain: ChainName;
  protocol: string;
  token?: string | null;
  amount?: number | null;
  destination?: string | null;
  metadata?: Record<string, unknown>;
}

/** Constraint kinds the planner understands (serde externally-tagged). */
export type ConstraintType =
  | { MaxSlippageBps: number }
  | { MaxGasCost: number }
  | { Deadline: number }
  | { MinAmount: number }
  | { MaxAmount: number }
  | { RequiredConfirmations: number }
  | { WhitelistedProtocols: string[] }
  | { BlacklistedProtocols: string[] };

/** A constraint that bounds how the plan may execute. */
export interface TrustIntentConstraint {
  constraint_type: ConstraintType;
  hard: boolean;
  description: string;
}

/**
 * High-level intent declared by an agent. The sidecar decomposes it into
 * sequential legs, routes it through the route selector, and starts it as a
 * durable workflow via POST /execute.
 */
export interface TrustIntent {
  description: string;
  agent_id: string;
  source_chain: ChainName;
  actions: TrustIntentAction[];
  constraints?: TrustIntentConstraint[];
}

/** Options for {@link BastionWorkflow.executeIntent}. */
export interface ExecuteIntentOptions {
  /** Insert a human-approval step before settling each leg. Defaults to true. */
  requireApproval?: boolean;
}

/** One leg of a decomposed execution plan. */
export interface PlanLegSummary {
  id: string;
  chain: ChainName;
  action_type: IntentActionType;
  protocol: string;
  status: string;
}

/** Response from POST /execute. */
export interface ExecuteIntentResponse {
  plan_id: string;
  workflow_id: string;
  status: string;
  selected_chain?: ChainName | null;
  routes: Array<{ chain: ChainName; score: number }>;
  legs: PlanLegSummary[];
  require_approval: boolean;
}

/** Response from GET /execute/:plan_id. */
export interface TrackedPlanResponse {
  plan_id: string;
  plan: Record<string, unknown>;
  workflow_id: string;
  workflow_status?: unknown;
  compensation_workflow_id?: string | null;
  compensation: {
    total_compensated: number;
    total_in_progress: number;
    total_failed: number;
  };
}

/** Response from POST /execute/:plan_id/compensate. */
export interface CompensationResponse {
  plan_id: string;
  status: string;
  compensation_workflow_id: string;
  compensating_legs: string[];
}


export interface WorkflowStep {
  id: string;
  activity: string;
  input: Record<string, unknown>;
  retry?: {
    maxAttempts: number;
    initialBackoffMs: number;
    backoffMultiplier: number;
    timeoutMs: number;
  };
  timeoutMs?: number;
  requiresApproval?: boolean;
  onFailure?: "halt" | "continue";
}

export interface WorkflowConfig {
  yaml?: string;
  steps?: WorkflowStep[];
  agentId?: string;
}

export interface WorkflowState {
  id: string;
  definition: string;
  status: string;
  current_step: number;
  step_states: Array<{
    step_id: string;
    status: string;
    input: Record<string, unknown>;
    output?: Record<string, unknown>;
    attempt: number;
    started_at?: number;
    completed_at?: number;
    error?: string;
  }>;
  created_at: number;
  updated_at: number;
  agent_id?: string;
}

export interface WorkflowEvent {
  type: string;
  id: string;
  step?: string;
  attempt?: number;
  timestamp: number;
  output?: Record<string, unknown>;
  error?: string;
  backoff_ms?: number;
  by?: string;
  definition?: string;
}

export class BastionWorkflow {
  private sidecar: BastionSidecar;

  constructor(sidecar: BastionSidecar) {
    this.sidecar = sidecar;
  }

  async start(config: WorkflowConfig): Promise<{ workflowId: string }> {
    if (config.yaml) {
      const r = await this.sidecar["request"]("POST", "/workflows", {
        yaml: config.yaml,
        agent_id: config.agentId,
      }) as { workflow_id: string };
      return { workflowId: r.workflow_id };
    }
    throw new Error("Only YAML workflows are currently supported via the REST API");
  }

  async state(workflowId: string): Promise<WorkflowState> {
    return this.sidecar["request"]("GET", `/workflows/${workflowId}`);
  }

  async replay(workflowId: string): Promise<WorkflowEvent[]> {
    return this.sidecar["request"]("GET", `/workflows/${workflowId}/events`);
  }

  async signal(workflowId: string, signal: "approve" | "cancel"): Promise<void> {
    await this.sidecar["request"]("POST", `/workflows/${workflowId}/signal`, {
      signal,
    });
  }

  async cancel(workflowId: string): Promise<void> {
    await this.sidecar["request"]("DELETE", `/workflows/${workflowId}`);
  }

  async list(agentId?: string): Promise<WorkflowState[]> {
    const query = agentId ? `?agent_id=${encodeURIComponent(agentId)}` : "";
    return this.sidecar["request"]("GET", `/workflows${query}`);
  }

  /**
   * Submit a TrustIntent for durable execution. The sidecar decomposes it
   * into sequential legs, scores routes across chains, and starts a durable
   * workflow (`simulate -> [approve] -> settle` per leg). When approval is
   * enabled (the default), poll {@link state} and call {@link signal} to
   * release each human-in-the-loop hold.
   */
  async executeIntent(
    intent: TrustIntent,
    opts: ExecuteIntentOptions = {},
  ): Promise<ExecuteIntentResponse> {
    return this.sidecar["request"]("POST", "/execute", {
      intent: { constraints: [], ...intent },
      require_approval: opts.requireApproval ?? true,
    });
  }

  /** Fetch a tracked execution plan with its live workflow status. */
  async plan(planId: string): Promise<TrackedPlanResponse> {
    return this.sidecar["request"]("GET", `/execute/${planId}`);
  }

  /**
   * Begin compensating a failed plan. Completed legs are unwound in reverse
   * order through a `compensate` workflow; the response names the legs being
   * undone. Resolves with HTTP 409 when the plan has nothing to compensate.
   */
  async compensate(planId: string): Promise<CompensationResponse> {
    return this.sidecar["request"]("POST", `/execute/${planId}/compensate`);
  }
}