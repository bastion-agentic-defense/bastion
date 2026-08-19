# Bastion - Solana Mainnet Readiness

> Living checklist for taking the `bastion-audit` Anchor program and its off-chain
> sidecar to **Solana mainnet-beta with real value**. Nothing in this document
> authorizes real-value traffic before the external audit sign-off (§7).

**Status legend:** ✅ done · 🟡 in progress · ⬜ not started

---

## 0. Scope

On-chain surface is intentionally minimal: one program, `bastion-audit`, providing
audit / identity / policy / circuit-breaker primitives (no financial instruments).
Program source: `crates/solana/programs/bastion-audit/src/lib.rs`. Spec (kept in
sync with code): `crates/solana/programs/bastion-audit/SPEC.md`.

The sidecar (`crates/sidecar`) is the only component that signs and submits
transactions to the program on mainnet (the audit-writer keypair).

---

## 1. Program hardening - code freeze targets

| Item | Status | Notes |
|---|---|---|
| Bind global authority to a config-provided admin (`initialize(admin)`), not first-caller | ✅ | `lib.rs` - sets `audit_state.authority = admin`; rejects `Pubkey::default()`. On mainnet `admin` = Squads vault, set atomically by the deploy script (§4). |
| Reputation bounded to `[0, 100]`, out-of-range rejected | ✅ | `update_agent_reputation` uses `MAX_REPUTATION = 100`; `InvalidReputation` on out-of-range. |
| Checked arithmetic on audit counters | ✅ | `log_audit` uses `checked_add` → `MathOverflow`. |
| `overflow-checks = true` in release/SBF profile | ✅ | root `Cargo.toml [profile.release]`. |
| SPEC ↔ code reconciliation (seeds, layouts, errors, reputation) | ✅ | `SPEC.md` rewritten to match implementation. |
| Named `POLICY_SEED` constant (no stray literals) | ✅ | `lib.rs`. |
| **Upgrade-authority-gated `initialize` (belt-and-suspenders)** | ⬜ | Recommended before real value: constrain `initialize` so only the program's upgrade authority (the Squads vault) can call it, eliminating the deploy→initialize front-run window entirely. See §1.1. Requires a `ProgramData` fixture in the test harness. |

### 1.1 Recommended upgrade-authority gate (to enable pre-audit)

Add to the `Initialize` accounts so only the upgrade authority can initialize:

```rust
#[account(constraint = program.programdata_address()? == Some(program_data.key()))]
pub program: Program<'info, crate::program::BastionAudit>,
#[account(constraint = program_data.upgrade_authority_address == Some(authority.key())
    @ BastionError::Unauthorized)]
pub program_data: Account<'info, ProgramData>,
```

Test harness change: inject a `ProgramData` account (owned by the BPF upgradeable
loader, `upgrade_authority_address = Some(payer)`) via `ProgramTest::add_account`
before `start()`. Until then, the operational mitigation (deploy + initialize in one
step, §4) plus the one-time `init` constraint is the interim control.

---

## 2. Tests & verification

| Item | Status | Notes |
|---|---|---|
| Negative-path tests actually submit txns and assert rejection | ✅ | `test_log_audit_unauthorized` / `test_log_audit_paused` now sign with the wrong key / while paused and assert the tx errors + no entry written. |
| `initialize` sets a distinct admin (not payer) | ✅ | `test_initialize_sets_distinct_admin`. |
| Reputation upper-bound rejection | ✅ | `test_reputation_upper_bound`. |
| Full `program-test` suite green (13 tests) | ✅ | `cargo test -p bastion-audit` (rebuild fixture `.so` first - see §2.1). |
| Fuzz `log_audit` / `set_policy` (trident or `cargo fuzz`) | ⬜ | Add before audit submission. |
| Wire program tests into CI `solana` job | 🟡 | `.github/workflows/ci.yml` already runs anchor build + test; add the fixture rebuild step. |

### 2.1 Rebuilding the test fixture
The integration tests load a prebuilt `.so`. After any program change:
```bash
cd crates/solana/programs/bastion-audit && cargo build-sbf
cp ../../../../target/deploy/bastion_audit.so tests/fixtures/bastion_audit.so
cargo test -p bastion-audit
```

---

## 3. Authority & key management (Squads multisig)

| Item | Status | Notes |
|---|---|---|
| Create Squads v4 multisig; record vault PDA | ⬜ | This vault is the program **admin** and **upgrade authority**. |
| Generate fresh mainnet program keypair | ⬜ | `solana-keygen new -o target/deploy/bastion_audit-keypair.json`. Never reuse the devnet ID (`A29V5MUVs73y7XBHHxPpPcAW7h4gGHupbDdwYSwA2n9D`). |
| Update `declare_id!` + all hard-coded copies | ⬜ | `lib.rs:3`, `crates/solana/Anchor.toml [programs.mainnet]` (placeholder present), `crates/sidecar/src/program_client.rs`, `apps/web/src/idl.json`, `packages/sdk/src/idl.json`, `apps/web/src/hooks/useBastionProgram.ts`, test `PROGRAM_ID`. |
| Separate the sidecar **audit-writer** keypair from the upgrade authority | ⬜ | The sidecar signer (`BASTION_KEYPAIR_PATH`) is `audit_state.authority` for `log_audit`; it is NOT the upgrade authority. Store via Fly.io secrets, never in repo. Document rotation. |

---

## 4. Mainnet config & deploy

| Item | Status | Notes |
|---|---|---|
| `[programs.mainnet]` block in `Anchor.toml` | ✅ | Placeholder ID present; replace with generated mainnet ID. |
| Verifiable build | 🟡 | `anchor build --verifiable` (or `solana-verify`) so the audit firm reproduces the binary hash. **Unblocked:** the program crate is now edition 2021 so Anchor 0.30.1 can parse its manifest (previously `edition = "2024"` broke `anchor build`/`deploy`/`test`). Produce the hash once the surface is frozen (§7). |
| Deploy | ⬜ | `anchor deploy --provider.cluster mainnet-beta` from a funded deployer (~2.5 SOL buffer). |
| Initialize atomically post-deploy | ⬜ | Call `initialize(admin = <Squads vault>)` in the same operational step (closes the front-run window with the interim control). |
| Transfer upgrade authority to the Squads vault | ⬜ | `solana program set-upgrade-authority <PROGRAM_ID> --new-upgrade-authority <VAULT>`; verify with `solana program show <PROGRAM_ID>`. |
| Regenerate + sync IDL | ⬜ | Update `packages/sdk` (bump **major**, currently 0.5.2) and `apps/web` IDLs to the mainnet ID. |

---

## 5. Sidecar / infra cutover

| Item | Status | Notes |
|---|---|---|
| Auth fails closed | ✅ | `crates/sidecar/src/auth.rs` - with `BASTION_REQUIRE_AUTH=1`, requests lacking DID auth or a valid `BASTION_API_KEY` are rejected. **Set `BASTION_REQUIRE_AUTH=1` on mainnet.** |
| Mutating endpoints auth-protected | ✅ | `/simulate`, `/api/v2/evaluate`, `/api/v2/simulate-evm`, delegate, delegation delete, stake unstake/claim, robot telemetry moved under the auth layer in `build_app`. |
| Structured logging | ✅ | `main.rs::init_tracing` - `RUST_LOG` controls level; `BASTION_LOG_JSON=1` for JSON (set on Fly.io). |
| Signer-path panic sweep | ✅ | `program_client.rs` only panics on a compile-time constant ID; the runtime submit path is panic-free. Remaining `unwrap`s are startup config or infallible response builders. |
| `/metrics` (Prometheus) + request-latency tracing | ⬜ | Add before go-live. |
| RPC cutover devnet → paid mainnet provider | ⬜ | `fly.toml` `SOLANA_RPC_URL` / `HELIUS_RPC_URL` still point at devnet; switch to a paid mainnet RPC. |
| Secrets via Fly.io (Helius/Alchemy/Grond keys, signer keypair) | ⬜ | Remove from env files; document rotation. |
| Deploy host rename (`bastion-agentique.fly.dev` → zkOS-Labs host/domain) | ⬜ | **Deferred.** The org rebrand (bastion-agentique → zkos-labs) intentionally kept the live Fly app name `bastion-agentique` and host `bastion-agentique.fly.dev` to avoid a DNS/infra migration. Revisit before mainnet: rename the Fly app or attach a custom domain, then update SDK/web defaults and `.well-known/*`. |
| `cargo audit` / `cargo deny` in CI + triage RUSTSEC items | ✅ | `cargo audit` runs in CI (`.github/workflows/ci.yml` `audit` job); 11 known Solana-SDK advisories allowlisted in `.cargo/audit.toml` and tracked in `SECURITY.md`. Any new advisory fails CI. Remaining: bump Solana SDK to clear the allowlist. |

---

## 6. Arcium (confidential evaluation) decision

The Arcium MXE path is scaffolded but wired to `NoopArciumClient` (always `Pass`,
empty signature) - `crates/arcium/src/client.rs`. **Do not ship a no-op as
"confidential."** Before mainnet, choose one and document it publicly:

- **(a)** Ship with local-eval fallback only and label Arcium as **"preview / not yet
  enforcing"** in `SECURITY.md`, the dashboard, and marketing; or
- **(b)** Block go-live of the confidential claim on a real MXE client + Arcis circuit
  (`crates/arcium/src/circuits/policy_evaluator.rs` is currently a stub).

Status: ✅ **Decision (a) — "preview / not enforcing".**

The runtime enforces this honestly:

- `arcium_enabled` defaults to `false` and now actually gates the wiring in
  `crates/sidecar/src/lib.rs` (with it off, the evaluator is
  `ArcumPolicyEvaluator::disabled(..)` with `arcium: None`).
- `/health` reports `confidential_compute: false`.
- The SDK refuses `execute({ privacy: "confidential" })` unless the runtime reports
  genuine MPC compute (`packages/sdk/src/execute.ts`).
- Option (b) — a real MXE client + Arcis circuits — is tracked as Epic B in
  `docs/ROADMAP.md` and stays behind the external-audit hard gate.

---

## 7. External audit - GO-LIVE GATE

| Item | Status | Notes |
|---|---|---|
| Freeze program surface | ⬜ | After §1–§2 complete. |
| Submit to a Solana-focused firm | ⬜ | e.g. OtterSec / Neodyme / Zellic. Provide the verifiable build hash. |
| Remediate findings + re-audit | ⬜ | |
| Publish report link in `SECURITY.md` | ⬜ | |

**HARD GATE:** no mainnet real-value traffic until the audit is signed off.

---

## 8. Go-live smoke test

1. `solana program show <PROGRAM_ID>` → upgrade authority == Squads vault.
2. Devnet dry-run: `initialize` → `log_audit` → `emergency_pause` → `emergency_resume`
   with the multisig as authority.
3. `POST /simulate` (authenticated) on the Fly sidecar with a real tx →
   audit entry lands on the mainnet program; confirm via the SDK / explorer.
4. All `.github/workflows/ci.yml` jobs green on `main`.
5. Lift the "alpha / not-production" banners only after audit sign-off.
