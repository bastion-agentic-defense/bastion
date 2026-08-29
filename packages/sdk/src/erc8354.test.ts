import { generatePrivateKey, privateKeyToAccount } from "viem/accounts";
import {
  commitPolicyAction,
  verdictDigest,
  verifyVerdict,
  consumeVerdict,
} from "./erc8354";
import type { PolicyAction, Verdict, VerdictAttestation } from "./erc8354";

const ADDR = `0x${"11".repeat(20)}` as `0x${string}`;
const B32 = (c: string) => `0x${c.repeat(32)}` as `0x${string}`; // 32 bytes = 64 hex chars

const ACTION: PolicyAction = {
  chainId: 1n,
  domainId: B32("0a"),
  agentId: 7n,
  target: ADDR,
  value: 1000n,
  callDataHash: B32("0e"),
  actionNonce: 3n,
};

const VERDICT: Verdict = {
  agentId: 7n,
  domainId: B32("0a"),
  policyRoot: B32("0b"),
  actionCommitment: B32("0c"),
  executor: ADDR,
  expiry: BigInt(Math.floor(Date.now() / 1000)) + 3600n,
  nullifier: B32("0d"),
  decision: 0,
  policyKind: 1,
};

describe("ERC-8354 wrappers", () => {
  it("commitPolicyAction is deterministic", () => {
    expect(commitPolicyAction(ACTION)).toBe(commitPolicyAction(ACTION));
  });

  it("commitPolicyAction is order-sensitive", () => {
    const swapped: PolicyAction = {
      ...ACTION,
      chainId: ACTION.value,
      value: ACTION.chainId,
    };
    expect(commitPolicyAction(swapped)).not.toBe(commitPolicyAction(ACTION));
  });

  it("verdictDigest is deterministic and decision-sensitive", () => {
    expect(verdictDigest(VERDICT)).toBe(verdictDigest(VERDICT));
    expect(verdictDigest({ ...VERDICT, decision: 1 })).not.toBe(
      verdictDigest(VERDICT),
    );
  });

  it("verifyVerdict accepts a valid attestation and rejects tampering", async () => {
    const account = privateKeyToAccount(generatePrivateKey());
    const executor = account.address;
    const verdict: Verdict = { ...VERDICT, executor };
    const digest = verdictDigest(verdict);
    const signature = await account.sign({ hash: digest });
    const deadline = BigInt(Math.floor(Date.now() / 1000)) + 3600n;
    const attestation: VerdictAttestation = {
      verdictDigest: digest,
      executor,
      deadline,
      signature,
    };

    expect(await verifyVerdict(verdict, attestation)).toBe(true);
    expect(await verifyVerdict({ ...verdict, decision: 1 }, attestation)).toBe(false);
  });

  it("consumeVerdict returns the nullifier and action commitment", () => {
    expect(consumeVerdict(VERDICT)).toEqual({
      nullifier: VERDICT.nullifier,
      actionCommitment: VERDICT.actionCommitment,
    });
  });
});
