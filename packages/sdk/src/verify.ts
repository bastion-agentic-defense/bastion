/**
 * Recompute verification for Bastion audit records.
 *
 * Implements the trustless-ai "Don't trust. Recompute." philosophy.
 * Every function here is pure and stateless - it reproduces the same
 * hash the Bastion Trust Runtime computed, from public inputs only.
 *
 * ERC-8281 (OCP): observation_digest = sha256("Decision:payload_hash")
 * ERC-8299 (WYRIWE): triple-hash input provenance
 *
 * @module verify
 */

import { createHash } from "crypto";

/** SHA-256 hex digest with 0x prefix. */
function sha256(data: Buffer | string): string {
  return "0x" + createHash("sha256").update(data).digest("hex");
}

/**
 * Recompute the ERC-8281 OCP observation digest from a policy decision
 * and the payload hash. Any third party can call this with public inputs
 * to verify the audit record was computed correctly.
 */
export function recomputeObservationDigest(
  decision: string,
  payloadHash: string,
): string {
  const observation = `${decision}:${payloadHash}`;
  return sha256(observation);
}

/**
 * ERC-8299 WYRIWE: binds what the agent asked for to what passed policy.
 *
 * rawInputHash = sha256(raw_user_input)
 * sanitizationPipelineHash = sha256(sanitization_cid || rawInputHash)
 * inputHash = sha256(sanitized_input)
 * wyriweHash = sha256(rawInputHash || sanitizationPipelineHash || inputHash)
 *
 * When the sanitization pipeline is identity (no transform), sanitized_input
 * equals raw_input and the triple-hash proves the agent's intent was not
 * modified before policy evaluation.
 */
export function recomputeWyriweHash(rawInput: string): string {
  const rawInputHash = sha256(rawInput);
  const sanitizationCid = "identity";
  const sanitizationPipelineHash = sha256(sanitizationCid + rawInputHash.slice(2));
  const inputHash = sha256(rawInput);

  return sha256(
    rawInputHash.slice(2) +
      sanitizationPipelineHash.slice(2) +
      inputHash.slice(2),
  );
}

/**
 * Verify an entire audit record from public inputs. Returns true if
 * both the OCP observation digest and WYRIWE hash match a recompute
 * from the provided raw input, decision, and payload hash.
 */
export function verifyAuditRecord(params: {
  rawInput: string;
  decision: string;
  payloadHash: string;
  expectedObservationDigest: string;
  expectedWyriweHash: string;
}): { valid: boolean; recomputed: { observationDigest: string; wyriweHash: string } } {
  const observationDigest = recomputeObservationDigest(params.decision, params.payloadHash);
  const wyriweHash = recomputeWyriweHash(params.rawInput);

  return {
    valid:
      observationDigest === params.expectedObservationDigest &&
      wyriweHash === params.expectedWyriweHash,
    recomputed: { observationDigest, wyriweHash },
  };
}
