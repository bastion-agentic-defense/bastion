# Archive — Retired Solana & Arcium Surface

Bastion has pivoted to **full EVM**. This document records what was retired,
where the code still lives, and why. Nothing here was deleted from the
repository or from git history — it was **un-wired from the active product**
(workspace, CI, dashboard, SDK, sidecar runtime) and is kept on disk + history
as the archive.

> Status: the items below are **archived / retired**, not maintained. Do not add
> new features to them. If you need the Solana program or the Arcium stub for
> historical reference, the files are at the paths below.

## 1. Solana on-chain program

| What | Where | Why retired |
|------|-------|-------------|
| Anchor program `bastion-audit` | `crates/solana/programs/bastion-audit/` | EVM contracts (`evm/`) are now the only on-chain enforcement surface |
| Program IDL | `crates/solana/programs/bastion-audit/idl/bastion_audit.json` | same |
| Anchor workspace | `crates/solana/Anchor.toml` | same |
| Devnet program | `A29V5MUVs73y7XBHHxPpPcAW7h4gGHupbDdwYSwA2n9D` (upgrade authority `E9PsSz9XWgNR3TmSC57NHC2ZxJzF5NmbrWsDKEe7A7yM`) | devnet-only; superseded |

The crate is **removed from the Cargo workspace `members`**, so it no longer
builds/tests as part of the active workspace. It remains pinned to
`edition = "2021"` (Anchor 0.30.1 rejects 2024) — that pin is now only relevant
if someone deliberately resurrects it.

## 2. Arcium confidential-compute stub

| What | Where | Why retired |
|------|-------|-------------|
| `bastion-arcium` crate (NoopArciumClient) | `crates/arcium/` | Replaced by on-chain ZK verdicts via **ERC-8354** (see `docs/ERCS.md`) |
| Sidecar Arcium wiring | (removed from `crates/sidecar/`) | no longer in the runtime |
| Arcium design docs | `docs/ARCIUM_ARCHITECTURE_DECISION.md`, `docs/ARCIUM_INTEGRATION.md`, `docs/ARCIUM_INTEGRATION_SPEC.md` | superseded by ERC-8354 |

The Arcium crate is **removed from the Cargo workspace `members`** and its
dependency from the sidecar is dropped. The confidential-policy capability it
was standing in for is now expressed natively on EVM via the
`IConfidentialPolicyVerdict` interface (ERC-8354), where the *policy* is proven
in zero knowledge off-chain and the *verdict* is verified on-chain — no
third-party MPC service required.

## 3. Solana SDK + dashboard surface

| What | Where | Why retired |
|------|-------|-------------|
| `BastionClient` (Anchor) | was `packages/sdk/src/index.ts` | SDK is now `@zkos-labs/bastion-sdk`, EVM + HTTP |
| Anchor IDL | was `packages/sdk/src/idl.json`, `apps/web/src/idl.json` | removed |
| Solana wallet stack (wallet-adapter) | was `apps/web/src/App.tsx` | dashboard is EVM-only (RainbowKit/wagmi) |
| Solana DID auth (Ed25519 via solana_sdk) | was `crates/sidecar/src/auth.rs`, `agents.rs` | replaced with ed25519-dalek; DID prefix is now `did:bastion:evm:` |

## 4. CI

The `solana` GitHub Actions job (Solana CLI + Anchor install) was removed from
`.github/workflows/ci.yml`. The 11 inherited Solana-SDK `cargo audit` advisories
were dropped from `.cargo/audit.toml` once `Cargo.lock` no longer contained the
Solana crates.

## Long-term history

The full Solana + Arcium implementation remains in git history. To inspect the
last maintained state, use `git log -- crates/solana crates/arcium`.
