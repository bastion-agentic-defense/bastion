import type { BastionClient } from "./index";
import { BastionSidecar } from "./sidecar";
import type {
  EvmTxParams,
  SimulateResponse,
  EvmSimulateResponse,
} from "./types";

/**
 * The unified Bastion runtime facade.
 *
 * Rather than choosing infrastructure, callers declare the trust guarantees they
 * want (`policy`, `privacy`, `settlement`) and Bastion composes the existing
 * firewall primitives — simulation, policy evaluation, and audit — behind a
 * single `execute()` call.
 *
 * This is a thin composition over {@link BastionSidecar} (and optionally
 * {@link BastionClient} for on-chain settlement); it introduces no new backend.
 */

/** Privacy guarantee requested for an action. */
export type Privacy = "public" | "confidential";

/** Settlement network for an action. */
export type Settlement = "solana" | "ethereum" | "base" | "celo";

const EVM_SETTLEMENTS: readonly Settlement[] = ["ethereum", "base", "celo"];

export interface BastionRuntimeConfig {
  /** Sidecar client used for policy evaluation, simulation, and audit. */
  sidecar: BastionSidecar;
  /** Optional on-chain client (used for Solana settlement / on-chain audit). */
  client?: BastionClient;
}

export interface ExecuteRequest {
  /** Intent label for the action, e.g. "swap". Passed to the firewall as intent. */
  action: string;
  /** Which network the action settles on. Selects the evaluation path. */
  settlement: Settlement;
  /** Desired privacy. Defaults to "public". "confidential" requires the runtime
   *  to be running genuine Arcium MPC — otherwise `execute` throws. */
  privacy?: Privacy;
  /** Named policy to evaluate against (reserved; currently informational —
   *  the sidecar applies its configured policy set). */
  policy?: string;
  /** The concrete payload to evaluate: a base64 Solana transaction (for
   *  `settlement: "solana"`) or EVM tx params (for EVM settlements). */
  transaction: string | EvmTxParams;
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
  simulation?: SimulateResponse | EvmSimulateResponse;
}

interface SidecarError {
  status?: number;
  body?: { error?: string; block_id?: string };
  message?: string;
}

export class Bastion {
  private sidecar: BastionSidecar;
  private client?: BastionClient;

  constructor(config: BastionRuntimeConfig) {
    this.sidecar = config.sidecar;
    this.client = config.client;
  }

  /**
   * Evaluate an action against the trust pipeline and return a decision.
   *
   * Composes: privacy enforcement → policy evaluation + simulation (per
   * settlement network) → verification. Returns `pass` / `block` / `pending_hitl`.
   */
  async execute(req: ExecuteRequest): Promise<ExecuteResult> {
    const privacy: Privacy = req.privacy ?? "public";

    // Honesty guard: never evaluate "confidential" in the clear. If the runtime
    // is not running genuine Arcium MPC, refuse rather than silently downgrade.
    if (privacy === "confidential") {
      const health = await this.sidecar.health();
      if (!health.confidential_compute) {
        throw new Error(
          "Confidential execution requested, but the Bastion runtime is not " +
            "running confidential (Arcium MPC) compute. Refusing to proceed " +
            "rather than evaluating in the clear.",
        );
      }
    }

    if (req.settlement === "solana") {
      return this.executeSolana(req, privacy);
    }
    if (EVM_SETTLEMENTS.includes(req.settlement)) {
      return this.executeEvm(req, privacy);
    }
    throw new Error(`Unsupported settlement network: ${req.settlement}`);
  }

  private async executeSolana(
    req: ExecuteRequest,
    privacy: Privacy,
  ): Promise<ExecuteResult> {
    if (typeof req.transaction !== "string") {
      throw new Error(
        "Solana settlement requires a base64-encoded transaction string",
      );
    }
    try {
      const simulation = await this.sidecar.simulate({
        transaction: req.transaction,
        intent: req.action,
      });
      return {
        decision: "pass",
        action: req.action,
        settlement: req.settlement,
        privacy,
        simulation,
      };
    } catch (err) {
      const e = err as SidecarError;
      // The sidecar signals a firewall decision with HTTP 403. Anything else
      // (network, 5xx, misconfiguration) is a genuine error and rethrows.
      if (e.status !== 403) throw err;
      const reason = e.body?.error ?? e.message;
      // A block_id means the action is held for human-in-the-loop approval.
      if (e.body?.block_id) {
        return {
          decision: "pending_hitl",
          action: req.action,
          settlement: req.settlement,
          privacy,
          reason,
          approvalId: e.body.block_id,
        };
      }
      return {
        decision: "block",
        action: req.action,
        settlement: req.settlement,
        privacy,
        reason,
      };
    }
  }

  private async executeEvm(
    req: ExecuteRequest,
    privacy: Privacy,
  ): Promise<ExecuteResult> {
    if (typeof req.transaction === "string") {
      throw new Error(
        "EVM settlement requires an EvmTxParams object, not a string",
      );
    }
    const simulation = await this.sidecar.simulateEvm({
      transaction: req.transaction,
      intent: req.action,
      chain: req.settlement,
      agentId: req.agentId,
    });
    let decision: ExecuteDecision;
    if (simulation.allowed) {
      decision = "pass";
    } else if (simulation.decision?.toLowerCase().includes("pend")) {
      decision = "pending_hitl";
    } else {
      decision = "block";
    }
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
