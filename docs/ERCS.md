# Bastion — Ethereum Standards (composed + draft)

Bastion composes existing standards rather than replacing them. This document is
the single source of truth for which standards Bastion integrates and their
status.

> ⚠️ **Draft status.** ERC-8354 and ERC-8380 are **draft / pre-consensus**
> proposals. The integrations below are experimental, pinned to a specific draft
> commit, and feature-flagged. Interfaces may change before finalization — treat
> the contract ABIs and SDK wrappers as draft-conformant, not stable.

## Draft standards integrated (this repo)

| ERC | Name | Draft PR | Draft commit | Status |
|-----|------|----------|--------------|--------|
| **ERC-8354** | Confidential Agent Policy Verdicts | [ethereum/ERCs#1919](https://github.com/ethereum/ERCs/pull/1919) | see `evm/src/erc8354/` header comment | experimental |
| **ERC-8380** | Unclonable Agent Execution Credentials | [ethereum/ERCs#1953](https://github.com/ethereum/ERCs/pull/1953) | see `evm/src/erc8380/` header comment | experimental |

### ERC-8354 — Confidential Agent Policy Verdicts

A pre-execution allow/deny verdict for agent actions, proven in zero knowledge
against a policy that is never disclosed on-chain. Bridges **ERC-8004** (agent
identity) and **ERC-7812** (evidence registry) with a verdict envelope that a
guard contract verifies locally.

- **`Verdict` envelope** — `agentId, domainId, policyRoot, actionCommitment,
  executor, expiry, nullifier, decision (0=DENY/1=ALLOW), policyKind`.
- **`IConfidentialPolicyVerdict`** — `verify`, `verdictDigest`, `consume`
  (two overloads incl. signed-relay), `isConsumed`; single-use
  `(domainId, nullifier)`.
- **`IPolicyGuarded`** — `policyDomain()`.
- **`IPolicyDomainRegistry`** — `domain`, `currentRoot`, `isRootAcceptable`.
- **`VerdictAttestation`** → ERC-7812, with `mechanism = keccak256("zk-secret-policy")`.
- **`PolicyAction.commit()`** binds a verdict to a specific action.

Bastion contracts: `evm/src/erc8354/BastionConfidentialVerdict.sol`,
`BastionPolicyDomainRegistry.sol`, `BastionGuardedExecutor.sol`.
SDK: `@zkos-labs/bastion-agentique` → `erc8354` module.
Proving is **off-chain** (Noir/barretenberg, see the spec's `assets/erc-8354/circuits`);
verification is **on-chain**.

### ERC-8380 — Unclonable Agent Execution Credentials

A single-use capability token for delegated agent execution. An orchestrator
issues a capability bound to an agent identity and a per-issuance secret salt;
spending it forces a nullifier derived from that salt into the open, so a cloned
agent (same salt → same nullifier) is rejected.

- **`Capability`** — `nullifier, capabilityCommitment, agentId, homeChainId,
  homeDomainId, capabilityIndex, actionCommitment, executor, expiry`.
- **`IUnclonableCredential`** — `issue`, `execute`, `isConsumed`,
  `highestIssuedIndex`; errors `CredentialAlreadySpent`, `CommitmentNotIssued`.
- `computeNullifier = keccak256("ERC-1953/nullifier/v1", salt)` — note the tag
  string is frozen as-is (the PR's assets predate the ERC-8380 number; the bytes
  are kept identical for compatibility, only naming is normalized to ERC-8380).
- `homeChainId` is an **acceptance check**, not part of the nullifier preimage
  (a nullifier in the preimage would fork per chain and allow one spend per chain).

Bastion contracts: `evm/src/erc8380/BastionUnclonableCredentialGuard.sol`,
`DomainRegistry.sol`. SDK: `@zkos-labs/bastion-agentique` → `erc8380` module.

### Known open questions (from the drafts, stated not hidden)

- **Root staleness = revocation delay** (ERC-8354): `isRootAcceptable` bounds how
  stale a committed policy root may be.
- **At-most-once, not exactly-once** (ERC-8380): two holders of the same salt
  race; the guard cannot rank them.
- **Proof-generation latency** (both): proving sits in the execution path.
- **Cross-chain nullifier mirroring** (ERC-8380): a credential spendable on
  either of two chains with no designated home is out of scope.
- **Relayed submission** (ERC-8380): `msg.sender == executor` means no gasless
  relayer without an added EIP-712 signature argument — deferred.

## Standards Bastion composes (stable)

| ERC / standard | Role in Bastion |
|----------------|-----------------|
| **ERC-8004** | Agent identity (NFT + EIP-712); `agentId` in both drafts above |
| **ERC-4337** | Account abstraction / UserOperation flow |
| **ERC-7579** | Validator module (BastionFirewall) |
| **ERC-7812** | Evidence registry (verdict attestations) |
| **EIP-712** | Typed structured data (audit entries, verdicts, capabilities) |
| **EAS / Sign Protocol / x402 / Pact** | Attestations + payments + network composition |
| **ERC-8126 / EIP-7702** | Value/AA composition (see `AGENTS.md`) |
