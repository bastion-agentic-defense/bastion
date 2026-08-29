import {
  encodeAbiParameters,
  keccak256,
  recoverAddress,
  type Address,
  type Hex,
} from "viem";

/**
 * ERC-8354 (draft) — confidential verdict wrappers.
 *
 * These pure helpers recompute the commitments and digests for Bastion's
 * confidential-verdict flow. They are chain-independent: the exact ABI encoding
 * order below is the spec's canonical ordering, so any off-chain recompute must
 * match the on-chain commitment.
 *
 * NOTE: ERC-8354 is a draft standard — the field ordering here tracks the current
 * draft commit and may change before finalization.
 */

/** A policy action whose commitment is signed into a verdict. */
export interface PolicyAction {
  chainId: bigint;       // uint256
  domainId: Hex;         // bytes32
  agentId: bigint;       // uint256
  target: Address;       // address
  value: bigint;         // uint256
  callDataHash: Hex;     // bytes32
  actionNonce: bigint;   // uint256
}

/** A confidential verdict over a policy action. */
export interface Verdict {
  agentId: bigint;          // uint256
  domainId: Hex;            // bytes32
  policyRoot: Hex;          // bytes32
  actionCommitment: Hex;    // bytes32
  executor: Address;        // address
  expiry: bigint;           // uint256
  nullifier: Hex;           // bytes32
  decision: 0 | 1;          // uint8
  policyKind: number;       // uint256
}

/** Attestation binding a verdict digest to an executor signature. */
export interface VerdictAttestation {
  verdictDigest: Hex;
  executor: Address;
  deadline: bigint;
  signature: Hex;
}

/**
 * Commit a policy action to its canonical hash.
 *
 * `keccak256(abi.encode(chainId, domainId, agentId, target, value, callDataHash,
 * actionNonce))` — this exact ordering must match the spec.
 */
export function commitPolicyAction(action: PolicyAction): Hex {
  return keccak256(
    encodeAbiParameters(
      [
        { type: "uint256" }, // chainId
        { type: "bytes32" }, // domainId
        { type: "uint256" }, // agentId
        { type: "address" }, // target
        { type: "uint256" }, // value
        { type: "bytes32" }, // callDataHash
        { type: "uint256" }, // actionNonce
      ],
      [
        action.chainId,
        action.domainId,
        action.agentId,
        action.target,
        action.value,
        action.callDataHash,
        action.actionNonce,
      ],
    ),
  );
}

/**
 * Compute the canonical digest of a verdict.
 *
 * `keccak256(abi.encode(agentId, domainId, policyRoot, actionCommitment,
 * executor, expiry, nullifier, decision, policyKind))`.
 */
export function verdictDigest(verdict: Verdict): Hex {
  return keccak256(
    encodeAbiParameters(
      [
        { type: "uint256" }, // agentId
        { type: "bytes32" }, // domainId
        { type: "bytes32" }, // policyRoot
        { type: "bytes32" }, // actionCommitment
        { type: "address" }, // executor
        { type: "uint256" }, // expiry
        { type: "bytes32" }, // nullifier
        { type: "uint8" },   // decision
        { type: "uint256" }, // policyKind
      ],
      [
        verdict.agentId,
        verdict.domainId,
        verdict.policyRoot,
        verdict.actionCommitment,
        verdict.executor,
        verdict.expiry,
        verdict.nullifier,
        verdict.decision,
        BigInt(verdict.policyKind),
      ],
    ),
  );
}

/**
 * Verify a verdict against its attestation.
 *
 * Recomputes the digest, checks it matches the attestation, confirms the
 * attestation is unexpired, and that the signature recovers to the executor.
 */
export async function verifyVerdict(
  verdict: Verdict,
  attestation: VerdictAttestation,
): Promise<boolean> {
  const digest = verdictDigest(verdict);
  if (digest !== attestation.verdictDigest) return false;
  if (attestation.executor.toLowerCase() !== verdict.executor.toLowerCase()) {
    return false;
  }

  const now = BigInt(Math.floor(Date.now() / 1000));
  if (verdict.expiry <= now) return false;
  if (attestation.deadline <= now) return false;

  try {
    const recovered = await recoverAddress({
      hash: digest,
      signature: attestation.signature,
    });
    return recovered.toLowerCase() === verdict.executor.toLowerCase();
  } catch {
    return false;
  }
}

/**
 * Data required to mark a verdict's nullifier as consumed on-chain.
 *
 * The nullifier is the consumption key; the action commitment identifies which
 * policy action the verdict authorized.
 */
export function consumeVerdict(
  verdict: Verdict,
): { nullifier: Hex; actionCommitment: Hex } {
  return {
    nullifier: verdict.nullifier,
    actionCommitment: verdict.actionCommitment,
  };
}
