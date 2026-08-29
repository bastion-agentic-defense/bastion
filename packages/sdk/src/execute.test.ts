import { Bastion } from "./execute";
import type { BastionSidecar } from "./sidecar";
import type { EvmTxParams } from "./types";

/** Build a Bastion runtime over a stubbed sidecar. */
function makeBastion(stub: Partial<BastionSidecar>): Bastion {
  return new Bastion({ sidecar: stub as unknown as BastionSidecar });
}

const EVM_TX: EvmTxParams = {
  from: "0x1",
  to: "0x2",
  value: "0x0",
  data: "0x",
};

describe("Bastion.execute - EVM settlement", () => {
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

  it("supports monad settlement", async () => {
    const bastion = makeBastion({
      simulateEvm: async () => ({ allowed: true, decision: "allowed" }),
    });
    const res = await bastion.execute({
      action: "swap",
      settlement: "monad",
      transaction: EVM_TX,
    });
    expect(res.decision).toBe("pass");
    expect(res.settlement).toBe("monad");
  });

  it("requires an EvmTxParams object", async () => {
    const bastion = makeBastion({});
    await expect(
      bastion.execute({
        action: "swap",
        settlement: "ethereum",
        transaction: "AQID" as unknown as EvmTxParams,
      }),
    ).rejects.toThrow(/EvmTxParams object/);
  });
});

describe("Bastion.execute - confidential privacy guard", () => {
  it("always refuses confidential now that Arcium is retired", async () => {
    const bastion = makeBastion({
      simulateEvm: async () => ({ allowed: true, decision: "allowed" }),
    });
    await expect(
      bastion.execute({
        action: "swap",
        settlement: "ethereum",
        privacy: "confidential",
        transaction: EVM_TX,
      }),
    ).rejects.toThrow(/Confidential execution is retired/);
  });
});
