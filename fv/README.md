# Formal Verification

Bastion's formal verification layer. Proves correctness of the Trust Runtime's
core components using three complementary approaches.

## Layers

### Layer 1: Policy Engine (TLA+)

`fv/policy-engine/PolicyEvaluator.tla` — formal specification of the
`PolicyEvaluator::evaluate()` algorithm in TLA+. Model-checked with Apalache.

**Theorems proven:**

| Theorem | What it proves |
|---------|---------------|
| RuleOrdering | First blocking rule determines outcome, short-circuits correctly |
| Completeness | If all rules pass, result is Pass |
| Determinism | Same inputs always produce the same decision |
| NoSilentSkip | No rule is silently skipped during evaluation |
| HITLPrecedence | HITL takes precedence over Block when both apply |

**Usage:**
```bash
apalache-mc check --config=fv/policy-engine/PolicyEvaluator.cfg fv/policy-engine/PolicyEvaluator.tla
```

### Layer 2: EVM Contracts (Certora)

`fv/evm/` — Certora Prover specifications for on-chain contracts.

| Contract | Spec | Invariants |
|----------|------|-----------|
| BastionAudit | `BastionAudit.spec` | Append-only, firewall gate, AnchorProof emission, entry uniqueness, agent indexing |
| BastionPolicy | `BastionPolicy.spec` | Unregistered blocked, value cap, cooldown, deletion completeness, determinism |
| BastionConfidentialGate | `BastionConfidentialGate.spec` | Dual-layer gate, commitment mismatch, preflight consistency |

**Usage:**
```bash
certoraRun fv/evm/BastionAudit.conf
certoraRun fv/evm/BastionPolicy.conf
certoraRun fv/evm/BastionConfidentialGate.conf
```

### Layer 3: TrustPolicy Mapping (Property Tests)

`crates/policy-engine/tests/property_tests.rs` — Rust property tests proving
the TrustPolicy YAML to PolicyRule mapping is lossless.

See `fv/TRUSTPOLICY_MAPPING.md` for the full mapping table and invariants.

**Usage:**
```bash
cargo test -p bastion-policy-engine --test property_tests
```

## Philosophy

Vitalik's "focused verification" thesis: *verify the code so users don't need
to trust the developer — they only need to check the statement that was proven.*

Bastion's FV layer applies this to trust-critical components:

- The policy engine (must always produce correct decisions)
- The on-chain audit trail (must be append-only and firewalled)
- The TrustPolicy mapping (must be lossless)

What is NOT formally verified (by design):
- The simulation engine (depends on live chain state — covered by replay regression tests)
- The HTTP server (standard Axum infrastructure — covered by OZ's verified dependencies)
- The background scanner (simple interval polling — unit tests sufficient)
