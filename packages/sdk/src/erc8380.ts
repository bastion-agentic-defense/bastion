import {
  encodeAbiParameters,
  keccak256,
  type Address,
  type Hex,
} from "viem";

/**
 * ERC-8380 (draft) — unclonable capability credential wrappers.
 *
 * These pure helpers recompute the nullifier and capability commitment for a
 * capability credential. The exact ABI encoding order is the spec's canonical
 * ordering and must match the on-chain commitment.
 *
 * NOTE: ERC-8380 is a draft standard — field ordering tracks the current draft
 * commit and may change before finalization.
 */

/** An issued capability credential. */
export interface Capability {
  nullifier: Hex;             // bytes32
  capabilityCommitment: Hex;  // bytes32
  agentId: bigint;            // uint256
  homeChainId: bigint;        // uint256
  homeDomainId: Hex;          // bytes32
  capabilityIndex: bigint;    // uint256
  actionCommitment: Hex;      // bytes32
  executor: Address;          // address
  expiry: bigint;             // uint256
}

export interface CapabilityInputs {
  agentId: bigint;
  homeChainId: bigint;
  homeDomainId: Hex;
  capabilityIndex: bigint;
  actionCommitment: Hex;
  executor: Address;
  expiry: bigint;
}

/**
 * Compute the capability nullifier.
 *
 * `keccak256(abi.encode(agentId, homeChainId, homeDomainId, capabilityIndex))`.
 */
export function computeNullifier(inputs: {
  agentId: bigint;
  homeChainId: bigint;
  homeDomainId: Hex;
  capabilityIndex: bigint;
}): Hex {
  return keccak256(
    encodeAbiParameters(
      [
        { type: "uint256" }, // agentId
        { type: "uint256" }, // homeChainId
        { type: "bytes32" }, // homeDomainId
        { type: "uint256" }, // capabilityIndex
      ],
      [
        inputs.agentId,
        inputs.homeChainId,
        inputs.homeDomainId,
        inputs.capabilityIndex,
      ],
    ),
  );
}

/**
 * Compute the capability commitment that binds a nullifier to an authorized
 * action.
 *
 * `keccak256(abi.encode(nullifier, actionCommitment, executor))`.
 */
export function computeCapabilityCommitment(inputs: {
  nullifier: Hex;
  actionCommitment: Hex;
  executor: Address;
}): Hex {
  return keccak256(
    encodeAbiParameters(
      [
        { type: "bytes32" }, // nullifier
        { type: "bytes32" }, // actionCommitment
        { type: "address" }, // executor
      ],
      [inputs.nullifier, inputs.actionCommitment, inputs.executor],
    ),
  );
}

/**
 * Issue a capability credential: derives the nullifier and commitment and
 * assembles the full {@link Capability}.
 */
export function issueCapability(inputs: CapabilityInputs): Capability {
  const nullifier = computeNullifier({
    agentId: inputs.agentId,
    homeChainId: inputs.homeChainId,
    homeDomainId: inputs.homeDomainId,
    capabilityIndex: inputs.capabilityIndex,
  });
  const capabilityCommitment = computeCapabilityCommitment({
    nullifier,
    actionCommitment: inputs.actionCommitment,
    executor: inputs.executor,
  });
  return {
    nullifier,
    capabilityCommitment,
    agentId: inputs.agentId,
    homeChainId: inputs.homeChainId,
    homeDomainId: inputs.homeDomainId,
    capabilityIndex: inputs.capabilityIndex,
    actionCommitment: inputs.actionCommitment,
    executor: inputs.executor,
    expiry: inputs.expiry,
  };
}

/**
 * Data required to exercise a capability on-chain. The nullifier is the
 * one-time consumption key; the commitment proves the authorized action.
 */
export function executeCapability(
  capability: Capability,
): { nullifier: Hex; capabilityCommitment: Hex; actionCommitment: Hex } {
  return {
    nullifier: capability.nullifier,
    capabilityCommitment: capability.capabilityCommitment,
    actionCommitment: capability.actionCommitment,
  };
}

/**
 * Whether a capability has been consumed (expired). The on-chain contract also
 * tracks nullifier usage; this is the local expiry check.
 */
export function isConsumed(capability: Capability): boolean {
  const now = BigInt(Math.floor(Date.now() / 1000));
  return capability.expiry <= now;
}
