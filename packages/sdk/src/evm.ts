import type {
  Abi,
  Address,
  Chain,
  Hex,
  PublicClient,
  WalletClient,
} from "viem";

// Committed contract ABIs. These are copied verbatim from apps/web/src/abi/*.ts
// (the single source of truth, regenerated from the forge build) so the SDK's
// function/event names never drift from the on-chain contracts. We intentionally
// do NOT re-declare parseAbi([...]) strings here.
import AuditAbiJson from "./abi/BastionAudit";
import PolicyAbiJson from "./abi/BastionPolicy";
import FirewallAbiJson from "./abi/BastionFirewall";

const AUDIT_ABI = AuditAbiJson as Abi;
const POLICY_ABI = PolicyAbiJson as Abi;
const FIREWALL_ABI = FirewallAbiJson as Abi;

/** Deployed Bastion EVM contract addresses. */
export interface BastionEVMContracts {
  /** BastionAudit (EIP-712 immutable audit trail). */
  audit: Address;
  /** BastionPolicy (per-agent rules). */
  policy: Address;
  /** BastionFirewall (ERC-7579 validator). Optional. */
  firewall?: Address;
  /** BastionRegistry (agent/target directory). Optional. */
  registry?: Address;
  /** BastionERC8004Registry (agent identity). Optional. */
  erc8004?: Address;
}

export interface BastionEVMClientConfig {
  /** viem public client for the target chain (reads). */
  publicClient: PublicClient;
  /** viem wallet client for the target chain (writes). Optional for read-only use. */
  walletClient?: WalletClient;
  /** The chain the contracts are deployed on. */
  chain: Chain;
  /** Deployed contract addresses. */
  contracts: BastionEVMContracts;
}

/** `IBastionPolicy.Policy` struct, matching the ABI layout. */
export interface BastionPolicy {
  agent: Address;
  isActive: boolean;
  maxValuePerTx: bigint;
  maxGasPerTx: bigint;
  dailyTxLimit: bigint;
  cooldownSeconds: bigint;
  allowedTargets: readonly Address[];
  allowedSelectors: readonly Hex[];
  extraData: Hex;
}

/** `IBastionAudit.AuditEntry` struct, matching the ABI layout. */
export interface BastionAuditEntry {
  id: Hex;
  agent: Address;
  target: Address;
  selector: Hex;
  value: bigint;
  gasUsed: bigint;
  allowed: boolean;
  reason: Hex;
  timestamp: bigint;
  blockNumber: bigint;
  signature: Hex;
}

/**
 * viem-based client for the Bastion EVM contracts.
 *
 * Reads use the provided {@link PublicClient}; writes use the provided
 * {@link WalletClient}. Construct one instance per chain (chain + addresses are
 * immutable per instance).
 */
export class BastionEVMClient {
  readonly chain: Chain;
  readonly contracts: BastionEVMContracts;
  private publicClient: PublicClient;
  private walletClient?: WalletClient;

  constructor(config: BastionEVMClientConfig) {
    this.publicClient = config.publicClient;
    this.walletClient = config.walletClient;
    this.chain = config.chain;
    this.contracts = config.contracts;
  }

  /** Total number of audit entries recorded on-chain. */
  async getEntryCount(): Promise<bigint> {
    return this.publicClient.readContract({
      address: this.contracts.audit,
      abi: AUDIT_ABI,
      functionName: "getEntryCount",
    }) as Promise<bigint>;
  }

  /** Read a single audit entry by its bytes32 id. */
  async readAuditEntry(entryId: Hex): Promise<BastionAuditEntry> {
    return this.publicClient.readContract({
      address: this.contracts.audit,
      abi: AUDIT_ABI,
      functionName: "getEntry",
      args: [entryId],
    }) as Promise<BastionAuditEntry>;
  }

  /** Read the policy for an agent address. */
  async readPolicy(agent: Address): Promise<BastionPolicy> {
    return this.publicClient.readContract({
      address: this.contracts.policy,
      abi: POLICY_ABI,
      functionName: "getPolicy",
      args: [agent],
    }) as Promise<BastionPolicy>;
  }

  /** Write (set) the policy for an agent address. Requires a wallet client. */
  async writePolicy(agent: Address, policy: BastionPolicy): Promise<Hex> {
    return this.writeContract({
      address: this.contracts.policy,
      abi: POLICY_ABI,
      functionName: "setPolicy",
      args: [agent, policy],
    });
  }

  /** Evaluate a transaction against the per-agent policy (view function). */
  async validate(
    agent: Address,
    target: Address,
    value: bigint,
    callData: Hex,
  ): Promise<{ allowed: boolean; reason: Hex }> {
    return this.publicClient.readContract({
      address: this.contracts.policy,
      abi: POLICY_ABI,
      functionName: "checkTransaction",
      args: [agent, target, value, callData],
    }) as Promise<{ allowed: boolean; reason: Hex }>;
  }

  /** Whether the firewall validator is currently paused. */
  async isPaused(): Promise<boolean> {
    return this.publicClient.readContract({
      address: this.requireFirewall(),
      abi: FIREWALL_ABI,
      functionName: "paused",
    }) as Promise<boolean>;
  }

  /** Pause the firewall validator. Requires a wallet client. */
  async pause(): Promise<Hex> {
    return this.writeContract({
      address: this.requireFirewall(),
      abi: FIREWALL_ABI,
      functionName: "pause",
    });
  }

  /** Resume the firewall validator. Requires a wallet client. */
  async unpause(): Promise<Hex> {
    return this.writeContract({
      address: this.requireFirewall(),
      abi: FIREWALL_ABI,
      functionName: "unpause",
    });
  }

  private requireFirewall(): Address {
    if (!this.contracts.firewall) {
      throw new Error(
        "BastionEVMClient: contracts.firewall is required for this operation",
      );
    }
    return this.contracts.firewall;
  }

  private async writeContract(params: {
    address: Address;
    abi: Abi;
    functionName: string;
    args?: readonly unknown[];
  }): Promise<Hex> {
    if (!this.walletClient) {
      throw new Error(
        "BastionEVMClient: a WalletClient is required for write operations",
      );
    }
    const account = this.walletClient.account;
    if (!account) {
      throw new Error(
        "BastionEVMClient: walletClient has no account configured",
      );
    }
    return this.walletClient.writeContract({
      address: params.address,
      abi: params.abi,
      functionName: params.functionName as never,
      args: params.args,
      account,
      chain: this.chain,
    });
  }
}
