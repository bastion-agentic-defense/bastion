// Bastion SDK — EVM + HTTP client for the Bastion Programmable Trust Runtime.
//
// The SDK is EVM-only after the full-EVM pivot: the Solana Anchor program client
// (`BastionClient`, `idl.json`) was removed. EVM contract access is via
// `BastionEVMClient` (viem); policy evaluation/simulation/audit via the HTTP
// `BastionSidecar`; durable execution via `BastionWorkflow`.

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
