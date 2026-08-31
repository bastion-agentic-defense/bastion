// Chain-agnostic + EVM/HTTP types. The Solana Anchor account types (AuditState,
// AuditEntry, Agent, Policy, event payloads, Pubkey) were removed with the full-EVM
// pivot — see git history for the Anchor-era definitions.

export const AGENT_CAPABILITIES = {
  TRANSFER: 1 << 0,
  SWAP: 1 << 1,
  NFT_MINT: 1 << 2,
  NFT_TRANSFER: 1 << 3,
  STAKE: 1 << 4,
  DELEGATE: 1 << 5,
  CREATE_PROGRAM: 1 << 6,
} as const;

export type AgentCapability = typeof AGENT_CAPABILITIES[keyof typeof AGENT_CAPABILITIES];

export const DECISION = {
  ALLOWED: 0,
  BLOCKED: 1,
  PENDING: 2,
} as const;

export type Decision = typeof DECISION[keyof typeof DECISION];

// ── Sidecar HTTP types ──────────────────────────────────────────────────────

export interface SidecarConfig {
  /** Base URL of the sidecar, e.g. "https://bastion-agentique.fly.dev/" */
  baseUrl: string;
  /** Optional API key sent as X-API-Key header */
  apiKey?: string;
}

export interface SidecarAuditEntry {
  timestamp: number;
  transaction_id?: string;
  transaction_signature?: string;
  decision: string;
  result: "allowed" | "blocked" | "pending";
  reasoning: string;
  intent?: string;
  simulation_logs?: string[];
}

export interface LogsQuery {
  limit?: number;
  offset?: number;
  transaction_id?: string;
  signature?: string;
  result?: "allowed" | "blocked" | "pending";
}

export interface LogsResponse {
  total: number;
  offset: number;
  limit: number;
  entries: SidecarAuditEntry[];
}

export interface SidecarPolicy {
  max_sol_per_tx?: number;
  max_balance_drain_lamports?: number;
  rate_limit_per_minute?: number;
  allowed_programs: string[];
  blocked_addresses: string[];
  simulation_checks_enabled: boolean;
}

export interface HealthResponse {
  status: string;
  uptime_seconds: number;
  db_healthy: boolean;
  db_size_bytes: number;
}

export interface OverrideRequest {
  block_id: string;
  action: "ALLOW" | "REJECT";
}

export interface CircuitBreakerStatus {
  engaged: boolean;
}

// ── Background trust scanner ─────────────────────────────────────────────────

/** The category of violation a scan finding reports. */
export type ScanFindingKind =
  | "expired_approval"
  | "expired_delegation"
  | "policy_drift"
  | "unsettled_transaction";

/** A single violation observed by the background scanner. */
export interface ScanFinding {
  kind: ScanFindingKind;
  /** Approval id, agent DID, policy name, or plan id. */
  id: string;
  detail: string;
}

/** Result of a background trust scan (camelCase as served by the sidecar). */
export interface ScanResult {
  timestamp: number;
  expiredApprovals: number;
  expiredDelegations: number;
  policyDrifts: number;
  unsettledTransactions: number;
  findings: ScanFinding[];
}

/** Response from GET /scan/results. */
export interface ScanResultsResponse {
  last_scan: ScanResult | null;
}

// ── EVM Simulation types ─────────────────────────────────────────────────────

export interface EvmTxParams {
  from: string;
  to: string;
  value?: string;
  data?: string;
  gas?: string;
  gasPrice?: string;
  maxFeePerGas?: string;
  maxPriorityFeePerGas?: string;
  nonce?: string;
}

export interface EvmSimulateRequest {
  transaction: EvmTxParams;
  intent?: string;
  chain?: string;
  agentId?: string;
}

export interface EvmSimulateResponse {
  allowed: boolean;
  decision: string;
  reason?: string;
  simulation_result?: {
    logs: string[];
    error?: unknown;
    balance_changes?: Record<string, number>;
    simulation_hash?: number[];
  };
  risk_score?: number;
  risk_summary?: string;
}

// ── Solana Simulation types (multichain settlement) ──────────────────────────

export interface SolanaSimulateRequest {
  /** Destination pubkey. */
  to: string;
  /** Amount in lamports. */
  amount?: number;
  /** Optional serialized (base58) Solana transaction for `simulateTransaction`. */
  transaction?: string;
  intent?: string;
  agentId?: string;
}

export interface SolanaSimulateResponse {
  allowed: boolean;
  decision: string;
  reason?: string;
  simulation_result?: {
    logs: string[];
    error?: unknown;
    balance_changes?: Record<string, number>;
    simulation_hash?: number[];
  };
  risk_score?: number;
  risk_summary?: string;
}

// ── SSE Events ────────────────────────────────────────────────────────────────

export interface SseEvent {
  type: string;
  data: unknown;
  id?: string;
}

export class BastionEventStream {
  private abortController: AbortController;
  private url: string;
  private headers: Record<string, string>;

  constructor(url: string, abortController: AbortController, headers: Record<string, string> = {}) {
    this.url = url;
    this.abortController = abortController;
    this.headers = headers;
  }

  private async *streamEvents(): AsyncGenerator<SseEvent> {
    const response = await fetch(this.url, {
      headers: {
        Accept: "text/event-stream",
        ...this.headers,
      },
      signal: this.abortController.signal,
    });

    if (!response.ok || !response.body) {
      throw new Error(`SSE connection failed: ${response.status}`);
    }

    const reader = response.body.getReader();
    const decoder = new TextDecoder();
    let buffer = "";

    try {
      while (true) {
        const { done, value } = await reader.read();
        if (done) break;

        buffer += decoder.decode(value, { stream: true });
        const lines = buffer.split("\n");
        buffer = lines.pop() ?? "";

        let eventType = "message";
        let eventData = "";
        let eventId: string | undefined;

        for (const line of lines) {
          const trimmed = line.trim();
          if (!trimmed) {
            if (eventData) {
              yield {
                type: eventType,
                data: tryParseJson(eventData),
                id: eventId,
              };
            }
            eventType = "message";
            eventData = "";
            eventId = undefined;
            continue;
          }

          if (trimmed.startsWith("event: ")) {
            eventType = trimmed.slice(7);
          } else if (trimmed.startsWith("data: ")) {
            eventData = trimmed.slice(6);
          } else if (trimmed.startsWith("id: ")) {
            eventId = trimmed.slice(4);
          }
        }
      }
    } finally {
      reader.releaseLock();
    }
  }

  /** Iterate over SSE events (for use in for-await-of loops) */
  async *[Symbol.asyncIterator](): AsyncGenerator<SseEvent> {
    yield* this.streamEvents();
  }

  /** Subscribe with callbacks */
  on(eventType: string | undefined, callback: (data: unknown, event: SseEvent) => void): () => void {
    let cancelled = false;
    (async () => {
      for await (const sseEvent of this.streamEvents()) {
        if (cancelled) break;
        if (!eventType || sseEvent.type === eventType) {
          callback(sseEvent.data, sseEvent);
        }
      }
    })().catch(() => {
      /* stream closed */
    });

    return () => {
      cancelled = true;
      this.abortController.abort();
    };
  }

  close(): void {
    this.abortController.abort();
  }
}

function tryParseJson(value: string): unknown {
  try {
    return JSON.parse(value);
  } catch {
    return value;
  }
}
