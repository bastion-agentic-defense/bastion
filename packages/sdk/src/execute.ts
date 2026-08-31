import { BastionSidecar } from "./sidecar";
import type {
  EvmTxParams,
  EvmSimulateResponse,
  SolanaSimulateResponse,
} from "./types";

/**
 * The unified Bastion runtime facade.
 *
 * Rather than choosing infrastructure, callers declare the trust guarantees they
 * want (`policy`, `privacy`, `settlement`) and Bastion composes the existing
 * firewall primitives - simulation, policy evaluation, and audit - behind a
 * single `execute()` call.
 *
 * This is a thin composition over {@link BastionSidecar} for EVM + Solana
 * settlement; it introduces no new backend.
 */

/** Privacy guarantee requested for an action. */
export type Privacy = "public" | "confidential";

/** Settlement network for an action. */
export type Settlement =
  | "ethereum"
  | "base"
  | "celo"
  | "zksync"
  | "robinhood"
  | "monad"
  | "polygon"
  | "arbitrum"
  | "solana";

const EVM_SETTLEMENTS: readonly Settlement[] = [
  "ethereum",
  "base",
  "celo",
  "zksync",
  "robinhood",
  "monad",
  "polygon",
  "arbitrum",
];

export interface BastionRuntimeConfig {
  /** Sidecar client used for policy evaluation, simulation, and audit. */
  sidecar: BastionSidecar;
}

export interface ExecuteRequest {
  /** Intent label for the action, e.g. "swap". Passed to the firewall as intent. */
  action: string;
  /** Which EVM network the action settles on. Selects the evaluation path. */
  settlement: Settlement;
  /** Desired privacy. Defaults to "public". "confidential" is retired (Arcium
   *  removed) and always throws; use ERC-8354 confidential verdicts instead. */
  privacy?: Privacy;
  /** Named policy to evaluate against (reserved; currently informational -
   *  the sidecar applies its configured policy set). */
  policy?: string;
  /** The concrete EVM transaction payload. Required for EVM settlements. */
  transaction?: EvmTxParams;
  /** Solana operation details. Required when `settlement === "solana"`. */
  solanaTx?: {
    /** Destination pubkey. */
    to: string;
    /** Amount in lamports. */
    amount?: number;
    /** Optional serialized (base58) transaction for `simulateTransaction`. */
    transaction?: string;
  };
  /** Optional agent identifier attributed to the action. */
  agentId?: string;
}

/** The trust decision for an executed action. */
export type ExecuteDecision = "pass" | "block" | "pending_hitl";

export interface ExecuteResult {
  decision: ExecuteDecision;
  action: string;
  settlement: Settlement;
  privacy: Privacy;
  /** Human-readable reason when blocked or pending. */
  reason?: string;
  /** Approval id to resolve a human-in-the-loop hold (when pending_hitl). */
  approvalId?: string;
  /** Raw simulation payload from the underlying firewall, when available. */
  simulation?: EvmSimulateResponse | SolanaSimulateResponse;
}

interface SidecarError {
  status?: number;
  body?: { error?: string; block_id?: string };
  message?: string;
}

export class Bastion {
  private sidecar: BastionSidecar;

  constructor(config: BastionRuntimeConfig) {
    this.sidecar = config.sidecar;
  }

  /**
   * Evaluate an action against the trust pipeline and return a decision.
   *
   * Composes: privacy enforcement → EVM policy evaluation + simulation →
   * verification. Returns `pass` / `block` / `pending_hitl`.
   */
  async execute(req: ExecuteRequest): Promise<ExecuteResult> {
    const privacy: Privacy = req.privacy ?? "public";

    // Honesty guard: confidential (Arcium MPC) evaluation is retired. Refuse
    // rather than silently downgrade to public. ERC-8354 confidential verdicts
    // are the replacement path (see docs/ERCS.md).
    if (privacy === "confidential") {
      throw new Error(
        "Confidential execution is retired (Arcium removed in the full-EVM " +
          "pivot). Use public execution, or ERC-8354 confidential verdicts.",
      );
    }

    if (req.settlement === "solana") {
      return this.executeSolana(req, privacy);
    }
    if (!EVM_SETTLEMENTS.includes(req.settlement)) {
      throw new Error(`Unsupported settlement network: ${req.settlement}`);
    }
    return this.executeEvm(req, privacy);
  }

  private async executeSolana(
    req: ExecuteRequest,
    privacy: Privacy,
  ): Promise<ExecuteResult> {
    if (!req.solanaTx) {
      throw new Error(
        "Solana settlement requires a `solanaTx` object (to, amount?)",
      );
    }
    const simulation = await this.sidecar.simulateSolana({
      ...req.solanaTx,
      intent: req.action,
      agentId: req.agentId,
    });
    const decision = this.decisionFrom(simulation.allowed, simulation.decision);
    return {
      decision,
      action: req.action,
      settlement: req.settlement,
      privacy,
      reason: simulation.reason,
      simulation,
    };
  }

  private decisionFrom(
    allowed: boolean,
    decisionText?: string,
  ): ExecuteDecision {
    if (allowed) return "pass";
    if (decisionText?.toLowerCase().includes("pend")) return "pending_hitl";
    return "block";
  }

  private async executeEvm(
    req: ExecuteRequest,
    privacy: Privacy,
  ): Promise<ExecuteResult> {
    if (typeof (req.transaction as unknown) === "string") {
      throw new Error(
        "EVM settlement requires an EvmTxParams object, not a string",
      );
    }
    if (!req.transaction) {
      throw new Error(
        "EVM settlement requires a `transaction` (EvmTxParams) object",
      );
    }
    const simulation = await this.sidecar.simulateEvm({
      transaction: req.transaction,
      intent: req.action,
      chain: req.settlement,
      agentId: req.agentId,
    });
    const decision = this.decisionFrom(simulation.allowed, simulation.decision);
    return {
      decision,
      action: req.action,
      settlement: req.settlement,
      privacy,
      reason: simulation.reason,
      simulation,
    };
  }
}
