# Bastion — EVM Mainnet Readiness

> Living checklist for taking the Bastion EVM contract suite (`evm/`) to mainnet on
> **Base, Celo, and Ethereum**. Nothing here authorizes real-value deployment before
> the external audit sign-off (§6).

**Status legend:** ✅ done · 🟡 in progress · ⬜ not started

Target chains: **Base**, **Celo**, **Ethereum mainnet** (Polygon dropped).
Stack: Solidity 0.8.28, Foundry, OpenZeppelin + Solady, `via_ir`. Contracts:
`BastionFirewall` (ERC-7579 validator), `BastionPolicy`, `BastionAudit` (EIP-712),
`BastionRegistry`, `BastionERC8004Registry` (soulbound identity), `BastionSidecar`.

---

## 1. Contract hardening — code freeze targets

| ID | Item | Status | Notes |
|---|---|---|---|
| B1 | `BastionAudit.record` access control | ✅ | Was callable by anyone (forge/spam audit entries). Now `Ownable` + `onlyFirewall`; owner (multisig) wires the firewall via `setFirewall`. |
| B2 | `validateUserOp` side-effect-free (ERC-4337) | ✅ | Validation is now `view`, returns `validationData` (0 / SIG_VALIDATION_FAILED) with no external state writes. Audit write + revert-on-block moved to a new execution-phase `enforce()`. |
| B3 | `_decodeCallData` bounds | ✅ | Requires `callData.length >= 68`; masks target to low 20 bytes; no underflow. |
| B4 | `BastionPolicy.setPolicy` unbounded loops | ✅ | Targets/selectors capped at 32×32 (`TooManyEntries`) so the allowlist matrix stays within block gas. |
| B12 | Firewall ↔ Policy selector-offset mismatch | ✅ | Latent bug found during hardening: firewall read the selector at offset 64 while the policy read it at offset 0, so the allowlist used the wrong selector. Unified on `[target][value][inner-calldata]`; firewall forwards the inner slice to the policy. |
| — | Review `BastionSidecar` / `BastionERC8004Registry` for the same access-control class | 🟡 | `BastionSidecar.fulfill` is `verifier`-gated; ERC8004 uses EIP-712 wallet binding. Include both in the audit scope; no unguarded state-writers found in review. |
| — | Remove "UNDER ACTIVE DEVELOPMENT / not production-ready" banners | ⬜ | Only after audit sign-off (§6). |

---

## 2. Tests & tooling to production bar

| Item | Status | Notes |
|---|---|---|
| Full suite green | ✅ | `forge test` — 62 tests (was 54). |
| Tests locking B1–B4 + B12 | ✅ | `test/BastionFirewallEnforce.t.sol` (validate/enforce split, decode bound, policy cap) + `test_Audit_RecordRevertsForNonFirewall`. |
| Branch coverage target | ⬜ | `forge coverage`; set a threshold and gate CI. |
| Invariant / fuzz tests | ⬜ | Policy limits, rate windows, audit append-only, firewall install/uninstall. |
| Gas snapshots in CI | ⬜ | `forge snapshot`; check `.gas-snapshot` in. |
| Extend CI | 🟡 | `evm/.github/workflows/test.yml` + root `ci.yml` `evm` job run build/test; add coverage + snapshot gates. |

---

## 3. Ownership & deploy pipeline

| Item | Status | Notes |
|---|---|---|
| Owner = per-chain Safe multisig, not raw deployer | ⬜ | `evm/script/DeployBastion.s.sol` currently passes `deployer` as owner; parametrize to the chain's Safe. |
| Deploy wires `audit.setFirewall(firewall)` | ✅ | Added to the deploy script (step 5). |
| Drop Polygon from targets | 🟡 | Keep base/celo/ethereum in `foundry.toml` `[rpc_endpoints]`; remove polygon usage from scripts/README. |
| Deterministic addresses (CREATE2) for cross-chain parity | ⬜ | Optional but recommended. |
| Etherscan/Basescan/Celoscan verification | ⬜ | Deploy with `--verify`; record addresses in §7. |

---

## 4. Sidecar EVM simulation parity

| Item | Status | Notes |
|---|---|---|
| Generalize simulator beyond Celo | ⬜ | Today only `CeloSimulator` (`crates/sidecar/src/simulation_evm.rs`, `eth_call` + balance diff). Drive per-chain RPC from the `Chain` enum (`crates/core/src/transaction/normalized.rs` — Base/Celo/Ethereum already present). |
| Real state-change prediction | ⬜ | Add `debug_traceCall` / state-override where the RPC supports it, not just balance delta. |
| Populate frontend EVM RPC config | ⬜ | `apps/web/src/lib/chains.ts` EVM entry has empty `rpcUrl`/`explorer`. |
| `/api/v2/simulate-evm` auth | ✅ | Now behind the sidecar auth layer (see `docs/MAINNET_READINESS.md` §5). |

---

## 5. Standards conformance

| Item | Status | Notes |
|---|---|---|
| ERC-7579 validator semantics (validation vs. execution) | ✅ | Corrected by B2 — `validateUserOp` no longer writes external storage. |
| Confirm `enforce()` wiring in the account/executor flow | ⬜ | The account must call `enforce()` during execution to record the audit entry; document the integration and add an end-to-end test with a mock ERC-7579 account. |
| ERC-4337 v0.7 `PackedUserOperation` compatibility | 🟡 | Struct matches; validate against a reference EntryPoint on testnet. |

---

## 6. External audit — GO-LIVE GATE

| Item | Status | Notes |
|---|---|---|
| Freeze contract surface | ⬜ | After §1–§2. |
| Audit all 6 contracts + deploy script | ⬜ | EVM-focused firm. |
| Remediate + re-audit | ⬜ | |
| Publish report; lift banners | ⬜ | |

**HARD GATE:** no mainnet real-value deployment until the audit is signed off.

---

## 7. Deployed addresses (fill in per chain post-deploy)

| Contract | Base | Celo | Ethereum |
|---|---|---|---|
| BastionAudit | — | — | — |
| BastionPolicy | — | — | — |
| BastionRegistry | — | — | — |
| BastionERC8004Registry | — | — | — |
| BastionFirewall | — | — | — |
| Owner (Safe) | — | — | — |

---

## 8. Go-live smoke test (per chain)

1. Testnet full-flow first (Base Sepolia / Celo Sepolia / ETH Sepolia).
2. `BastionAudit.record` reverts for a non-firewall caller.
3. `validateUserOp` returns 0 for an allowed op, `1` for a blocked/uninstalled op,
   with no state change; `enforce()` records the audit entry and reverts on block.
4. Confirm each contract's owner == the chain's Safe.
5. `forge test`, `forge coverage`, `forge snapshot` all green in CI.
