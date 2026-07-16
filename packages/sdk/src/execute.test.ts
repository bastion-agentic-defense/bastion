import { Bastion } from "./execute";
import type { BastionSidecar } from "./sidecar";

/** Build a Bastion runtime over a stubbed sidecar. */
function makeBastion(stub: Partial<BastionSidecar>): Bastion {
  return new Bastion({ sidecar: stub as unknown as BastionSidecar });
}

const SOLANA_TX = "AQID"; // arbitrary base64
const EVM_TX = { from: "0x1", to: "0x2", value: "0x0", data: "0x" };

describe("Bastion.execute — Solana settlement", () => {
  it("returns pass when simulation succeeds", async () => {
    const bastion = makeBastion({
      simulate: async () => ({ units_consumed: 1000, logs: [] }),
    });
    const res = await bastion.execute({
      action: "swap",
      settlement: "solana",
      transaction: SOLANA_TX,
    });
    expect(res.decision).toBe("pass");
    expect(res.simulation).toBeDefined();
  });

  it("maps a 403 without block_id to block", async () => {
    const bastion = makeBastion({
      simulate: async () => {
        throw Object.assign(new Error("blocked"), {
          status: 403,
          body: { error: "policy violation" },
        });
      },
    });
    const res = await bastion.execute({
      action: "swap",
      settlement: "solana",
      transaction: SOLANA_TX,
    });
    expect(res.decision).toBe("block");
    expect(res.reason).toBe("policy violation");
  });

  it("maps a 403 with block_id to pending_hitl", async () => {
    const bastion = makeBastion({
      simulate: async () => {
        throw Object.assign(new Error("held"), {
          status: 403,
          body: { error: "needs approval", block_id: "blk_123" },
        });
      },
    });
    const res = await bastion.execute({
      action: "swap",
      settlement: "solana",
      transaction: SOLANA_TX,
    });
    expect(res.decision).toBe("pending_hitl");
    expect(res.approvalId).toBe("blk_123");
  });

  it("rethrows non-403 errors", async () => {
    const bastion = makeBastion({
      simulate: async () => {
        throw Object.assign(new Error("boom"), { status: 500 });
      },
    });
    await expect(
      bastion.execute({
        action: "swap",
        settlement: "solana",
        transaction: SOLANA_TX,
      }),
    ).rejects.toThrow("boom");
  });

  it("requires a string transaction", async () => {
    const bastion = makeBastion({});
    await expect(
      bastion.execute({
        action: "swap",
        settlement: "solana",
        transaction: EVM_TX,
      }),
    ).rejects.toThrow(/base64-encoded transaction string/);
  });
});

describe("Bastion.execute — EVM settlement", () => {
  it("returns pass when allowed", async () => {
    const bastion = makeBastion({
      simulateEvm: async () => ({ allowed: true, decision: "allowed" }),
    });
    const res = await bastion.execute({
      action: "swap",
      settlement: "celo",
      transaction: EVM_TX,
    });
    expect(res.decision).toBe("pass");
  });

  it("returns block when not allowed", async () => {
    const bastion = makeBastion({
      simulateEvm: async () => ({
        allowed: false,
        decision: "blocked",
        reason: "reverts",
      }),
    });
    const res = await bastion.execute({
      action: "swap",
      settlement: "base",
      transaction: EVM_TX,
    });
    expect(res.decision).toBe("block");
    expect(res.reason).toBe("reverts");
  });

  it("requires an EvmTxParams object", async () => {
    const bastion = makeBastion({});
    await expect(
      bastion.execute({
        action: "swap",
        settlement: "ethereum",
        transaction: SOLANA_TX,
      }),
    ).rejects.toThrow(/EvmTxParams object/);
  });
});

describe("Bastion.execute — confidential privacy guard", () => {
  it("refuses confidential when the runtime is not confidential", async () => {
    const bastion = makeBastion({
      health: async () => ({
        status: "ok",
        uptime_seconds: 1,
        db_healthy: true,
        db_size_bytes: 0,
        confidential_compute: false,
      }),
    });
    await expect(
      bastion.execute({
        action: "swap",
        settlement: "solana",
        privacy: "confidential",
        transaction: SOLANA_TX,
      }),
    ).rejects.toThrow(/Refusing to proceed/);
  });

  it("proceeds when confidential compute is active", async () => {
    const bastion = makeBastion({
      health: async () => ({
        status: "ok",
        uptime_seconds: 1,
        db_healthy: true,
        db_size_bytes: 0,
        confidential_compute: true,
      }),
      simulate: async () => ({ units_consumed: 1 }),
    });
    const res = await bastion.execute({
      action: "swap",
      settlement: "solana",
      privacy: "confidential",
      transaction: SOLANA_TX,
    });
    expect(res.decision).toBe("pass");
    expect(res.privacy).toBe("confidential");
  });
});
