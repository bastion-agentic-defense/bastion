// Bastion SDK — multichain client for the Bastion Programmable Trust Runtime.
//
// Settles across EVM chains (via `BastionEVMClient` + `simulateEvm`) and Solana
// (via `simulateSolana` over the sidecar's TrustAdapter). The retired Solana
// Anchor *program client* (`BastionClient`, `idl.json`) is not revived; Solana
// execution has been re-added as a real settlement target through the HTTP
// sidecar. Policy evaluation/simulation/audit via `BastionSidecar`; durable
// execution via `BastionWorkflow`.

// Recompute verification (trustless-ai compatible)
export * as verify from "./verify";
export { BastionWorkflow } from "./workflow";
export type {
  WorkflowConfig,
  WorkflowState,
  WorkflowEvent,
  WorkflowStep,
  ChainName,
  IntentActionType,
  TrustIntentAction,
  ConstraintType,
  TrustIntentConstraint,
  TrustIntent,
  ExecuteIntentOptions,
  PlanLegSummary,
  ExecuteIntentResponse,
  TrackedPlanResponse,
  CompensationResponse,
} from "./workflow";

export { AGENT_CAPABILITIES, DECISION, BastionEventStream } from "./types";
export type {
  AgentCapability,
  Decision,
  SidecarConfig,
  EvmTxParams,
  EvmSimulateRequest,
  EvmSimulateResponse,
  SolanaSimulateRequest,
  SolanaSimulateResponse,
  SidecarAuditEntry,
  LogsQuery,
  LogsResponse,
  SidecarPolicy,
  HealthResponse,
  OverrideRequest,
  CircuitBreakerStatus,
  ScanResult,
  ScanFinding,
  ScanFindingKind,
  ScanResultsResponse,
  SseEvent,
} from "./types";

export { BastionSidecar } from "./sidecar";

export { Bastion } from "./execute";
export type {
  BastionRuntimeConfig,
  ExecuteRequest,
  ExecuteResult,
  ExecuteDecision,
  Privacy,
  Settlement,
} from "./execute";

// EVM contract client (viem)
export { BastionEVMClient } from "./evm";
export type {
  BastionEVMClientConfig,
  BastionEVMContracts,
  BastionPolicy,
  BastionAuditEntry,
} from "./evm";

// ERC-8354 confidential verdict wrappers
export {
  commitPolicyAction,
  verdictDigest,
  verifyVerdict,
  consumeVerdict,
} from "./erc8354";
export type {
  PolicyAction,
  Verdict,
  VerdictAttestation,
} from "./erc8354";

// ERC-8380 unclonable capability credential wrappers
export {
  computeNullifier,
  computeCapabilityCommitment,
  issueCapability,
  executeCapability,
  isConsumed,
} from "./erc8380";
export type { Capability, CapabilityInputs } from "./erc8380";
