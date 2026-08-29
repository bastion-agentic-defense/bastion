import {
  computeNullifier,
  computeCapabilityCommitment,
  issueCapability,
  executeCapability,
  isConsumed,
} from "./erc8380";
import type { CapabilityInputs } from "./erc8380";

const ADDR = `0x${"11".repeat(20)}` as `0x${string}`;
const B32 = (c: string) => `0x${c.repeat(32)}` as `0x${string}`; // 32 bytes = 64 hex chars

const INPUTS: CapabilityInputs = {
  agentId: 7n,
  homeChainId: 10143n,
  homeDomainId: B32("0a"),
  capabilityIndex: 1n,
  actionCommitment: B32("0c"),
  executor: ADDR,
  expiry: BigInt(Math.floor(Date.now() / 1000)) + 3600n,
};

describe("ERC-8380 wrappers", () => {
  it("computeNullifier is deterministic", () => {
    expect(
      computeNullifier({
        agentId: INPUTS.agentId,
        homeChainId: INPUTS.homeChainId,
        homeDomainId: INPUTS.homeDomainId,
        capabilityIndex: INPUTS.capabilityIndex,
      }),
    ).toBe(
      computeNullifier({
        agentId: INPUTS.agentId,
        homeChainId: INPUTS.homeChainId,
        homeDomainId: INPUTS.homeDomainId,
        capabilityIndex: INPUTS.capabilityIndex,
      }),
    );
  });

  it("computeCapabilityCommitment is deterministic", () => {
    const nullifier = B32("0d");
    expect(
      computeCapabilityCommitment({
        nullifier,
        actionCommitment: INPUTS.actionCommitment,
        executor: INPUTS.executor,
      }),
    ).toBe(
      computeCapabilityCommitment({
        nullifier,
        actionCommitment: INPUTS.actionCommitment,
        executor: INPUTS.executor,
      }),
    );
  });

  it("issueCapability assembles a consistent capability", () => {
    const capability = issueCapability(INPUTS);
    expect(capability.nullifier).toBe(
      computeNullifier({
        agentId: INPUTS.agentId,
        homeChainId: INPUTS.homeChainId,
        homeDomainId: INPUTS.homeDomainId,
        capabilityIndex: INPUTS.capabilityIndex,
      }),
    );
    expect(capability.capabilityCommitment).toBe(
      computeCapabilityCommitment({
        nullifier: capability.nullifier,
        actionCommitment: INPUTS.actionCommitment,
        executor: INPUTS.executor,
      }),
    );
    expect(capability.executor).toBe(INPUTS.executor);
  });

  it("executeCapability returns the consumption keys", () => {
    const capability = issueCapability(INPUTS);
    expect(executeCapability(capability)).toEqual({
      nullifier: capability.nullifier,
      capabilityCommitment: capability.capabilityCommitment,
      actionCommitment: capability.actionCommitment,
    });
  });

  it("isConsumed reflects expiry", () => {
    const capability = issueCapability(INPUTS);
    expect(isConsumed(capability)).toBe(false);
    expect(
      isConsumed({
        ...capability,
        expiry: BigInt(Math.floor(Date.now() / 1000)) - 1n,
      }),
    ).toBe(true);
  });
});
