export interface WorkflowStepDef {
  id: string;
  activity: string;
  input?: Record<string, unknown>;
  retry?: {
    maxAttempts?: number;
    initialBackoffMs?: number;
    backoffMultiplier?: number;
  };
  timeoutMs?: number;
  requiresApproval?: boolean;
  onFailure?: 'halt' | 'continue';
}

export interface WorkflowDefinition {
  name: string;
  steps: WorkflowStepDef[];
}

export type WorkflowStatus = 'running' | 'paused' | 'completed' | 'cancelled' | string;
export type StepStatus = 'pending' | 'running' | 'completed' | 'paused' | 'skipped' | string;

export interface StepState {
  step_id: string;
  status: StepStatus;
  input: unknown;
  output?: unknown;
  attempt: number;
  started_at?: number;
  completed_at?: number;
  error?: string;
}

export interface WorkflowState {
  id: string;
  definition: string;
  status: WorkflowStatus;
  current_step: number;
  step_states: StepState[];
  created_at: number;
  updated_at: number;
  agent_id?: string;
  tags: Record<string, string>;
}

export interface WorkflowEvent {
  type: string;
  id: string;
  step?: string;
  timestamp: number;
  [key: string]: unknown;
}

export class BastionWorkflow {
  private baseUrl: string;

  constructor(baseUrl: string) {
    this.baseUrl = baseUrl.replace(/\/$/, '');
  }

  async start(
    definition: WorkflowDefinition,
    agentId?: string,
  ): Promise<{ workflow_id: string }> {
    const res = await fetch(`${this.baseUrl}/workflows`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ definition: definition.name, agent_id: agentId }),
    });
    if (!res.ok) throw new Error(await res.text());
    return res.json() as Promise<{ workflow_id: string }>;
  }

  async startYaml(yaml: string, agentId?: string): Promise<{ workflow_id: string }> {
    const res = await fetch(`${this.baseUrl}/workflows/yaml`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ yaml, agent_id: agentId }),
    });
    if (!res.ok) throw new Error(await res.text());
    return res.json() as Promise<{ workflow_id: string }>;
  }

  async list(): Promise<WorkflowState[]> {
    const res = await fetch(`${this.baseUrl}/workflows`);
    if (!res.ok) throw new Error(await res.text());
    return res.json() as Promise<WorkflowState[]>;
  }

  async state(id: string): Promise<WorkflowState> {
    const res = await fetch(`${this.baseUrl}/workflows/${id}`);
    if (!res.ok) throw new Error(await res.text());
    return res.json() as Promise<WorkflowState>;
  }

  async events(id: string): Promise<WorkflowEvent[]> {
    const res = await fetch(`${this.baseUrl}/workflows/${id}/events`);
    if (!res.ok) throw new Error(await res.text());
    return res.json() as Promise<WorkflowEvent[]>;
  }

  async signal(id: string, signal: 'approve' | 'cancel'): Promise<void> {
    const res = await fetch(`${this.baseUrl}/workflows/${id}/signal`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ signal }),
    });
    if (!res.ok) throw new Error(await res.text());
  }

  async cancel(id: string): Promise<void> {
    const res = await fetch(`${this.baseUrl}/workflows/${id}`, {
      method: 'DELETE',
    });
    if (!res.ok) throw new Error(await res.text());
  }
}
