import { BastionSidecar } from "./sidecar";

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
}