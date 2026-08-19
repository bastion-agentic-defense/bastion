import { useCallback } from 'react';
import { useAccount, useWriteContract, usePublicClient } from 'wagmi';
import { decodeEventLog, type Abi, type Log } from 'viem';

// Committed contract ABIs (apps/web/src/abi/*.ts) — single source of truth for
// the deployed contracts. These are the actual forge build outputs, so the
// function/event names and signatures below match the on-chain bytecode.
import AuditAbiJson from '../abi/BastionAudit';
import PolicyAbiJson from '../abi/BastionPolicy';
import FirewallAbiJson from '../abi/BastionFirewall';

const AuditAbi = AuditAbiJson as Abi;
const PolicyAbi = PolicyAbiJson as Abi;
const FirewallAbi = FirewallAbiJson as Abi;

const CONTRACT_ADDRESSES = {
  audit: import.meta.env.VITE_BASTION_AUDIT_ADDRESS as string,
  policy: import.meta.env.VITE_BASTION_POLICY_ADDRESS as string,
  firewall: import.meta.env.VITE_BASTION_FIREWALL_ADDRESS as string,
  registry: import.meta.env.VITE_BASTION_REGISTRY_ADDRESS as string,
  erc8004: import.meta.env.VITE_BASTION_ERC8004_ADDRESS as string,
};

// BastionAudit.AuditRecorded(bytes32 indexed entryId, address indexed agent,
// address indexed target, bytes4 selector, bool allowed, uint256 timestamp).
const AuditRecordedEvent = {
  type: 'event',
  name: 'AuditRecorded',
  inputs: [
    { indexed: true, name: 'entryId', type: 'bytes32' },
    { indexed: true, name: 'agent', type: 'address' },
    { indexed: true, name: 'target', type: 'address' },
    { name: 'selector', type: 'bytes4' },
    { name: 'allowed', type: 'bool' },
    { name: 'timestamp', type: 'uint256' },
  ],
} as const;

type AuditRecordedArgs = {
  entryId?: `0x${string}`;
  agent?: `0x${string}`;
  target?: `0x${string}`;
  selector?: `0x${string}`;
  allowed?: boolean;
  timestamp?: bigint;
};

// IBastionPolicy.Policy struct (matches the ABI + on-chain layout).
type PolicyStruct = {
  agent: `0x${string}`;
  isActive: boolean;
  maxValuePerTx: bigint;
  maxGasPerTx: bigint;
  dailyTxLimit: bigint;
  cooldownSeconds: bigint;
  allowedTargets: readonly `0x${string}`[];
  allowedSelectors: readonly `0x${string}`[];
  extraData: `0x${string}`;
};

export interface AuditEntryData {
  id: string;
  timestamp: number;
  decision: 'ALLOWED' | 'BLOCKED' | 'PENDING';
  account: string;
  intent: string;
  reason: string;
}

export interface PolicyData {
  maxSolPerTx: number;
  rateLimit: number;
  allowedPrograms: string[];
}

export interface StatsData {
  total: number;
  allowed: number;
  blocked: number;
}

function decodeAuditRecorded(log: Log): AuditRecordedArgs | null {
  try {
    const decoded = decodeEventLog({ abi: AuditAbi, data: log.data, topics: log.topics });
    return (decoded.args ?? {}) as AuditRecordedArgs;
  } catch {
    return null;
  }
}

export function useBastionEVM() {
  const { address, isConnected } = useAccount();
  const publicClient = usePublicClient();
  const { writeContractAsync } = useWriteContract();

  const addresses = CONTRACT_ADDRESSES;

  const fetchStats = useCallback(async (): Promise<StatsData | null> => {
    if (!isConnected || !publicClient || !addresses.audit) return null;
    try {
      const total = (await publicClient.readContract({
        address: addresses.audit as `0x${string}`,
        abi: AuditAbi,
        functionName: 'getEntryCount',
      })) as bigint;

      const fromBlock = (await publicClient.getBlockNumber()) - 10000n;
      const logs = await publicClient.getLogs({
        address: addresses.audit as `0x${string}`,
        event: AuditRecordedEvent,
        fromBlock,
        toBlock: 'latest',
      });

      let allowed = 0;
      let blocked = 0;
      for (const log of logs) {
        const args = decodeAuditRecorded(log);
        if (!args) continue;
        if (args.allowed) allowed++;
        else blocked++;
      }

      return { total: Number(total), allowed, blocked };
    } catch (e) {
      console.error('fetchStats EVM error:', e);
      return null;
    }
  }, [isConnected, publicClient, addresses.audit]);

  const fetchPaused = useCallback(async (): Promise<boolean | null> => {
    if (!isConnected || !publicClient || !addresses.firewall) return null;
    try {
      const paused = (await publicClient.readContract({
        address: addresses.firewall as `0x${string}`,
        abi: FirewallAbi,
        functionName: 'paused',
      })) as boolean;
      return paused;
    } catch (e) {
      console.error('fetchPaused EVM error:', e);
      return null;
    }
  }, [isConnected, publicClient, addresses.firewall]);

  const fetchAuditEntries = useCallback(
    async (limit = 50): Promise<AuditEntryData[]> => {
      if (!isConnected || !publicClient || !addresses.audit) return [];
      try {
        const fromBlock = (await publicClient.getBlockNumber()) - 10000n;
        const allLogs = await publicClient.getLogs({
          address: addresses.audit as `0x${string}`,
          event: AuditRecordedEvent,
          fromBlock,
          toBlock: 'latest',
        });

        const entries: AuditEntryData[] = allLogs
          .slice(-limit)
          .reverse()
          .map((log) => {
            const args = decodeAuditRecorded(log) ?? {};
            return {
              id: args.entryId ?? log.transactionHash ?? log.logIndex?.toString() ?? '',
              timestamp: Number(args.timestamp ?? 0n),
              decision: args.allowed ? 'ALLOWED' : 'BLOCKED',
              account: args.agent ?? '',
              intent: `${args.target ?? ''}.${args.selector ?? ''}`,
              reason: args.allowed ? 'Policy passed' : 'Blocked by policy',
            };
          });

        return entries;
      } catch (e) {
        console.error('fetchAuditEntries EVM error:', e);
        return [];
      }
    },
    [isConnected, publicClient, addresses.audit],
  );

  const fetchPolicy = useCallback(async (): Promise<PolicyData | null> => {
    if (!isConnected || !publicClient || !addresses.policy || !address) return null;
    try {
      const policy = (await publicClient.readContract({
        address: addresses.policy as `0x${string}`,
        abi: PolicyAbi,
        functionName: 'getPolicy',
        args: [address],
      })) as unknown as PolicyStruct;

      return {
        maxSolPerTx: Number(policy.maxValuePerTx),
        rateLimit: Number(policy.dailyTxLimit),
        allowedPrograms: policy.allowedSelectors.map((s) => String(s)),
      };
    } catch (e) {
      console.error('fetchPolicy EVM error:', e);
      return null;
    }
  }, [isConnected, publicClient, addresses.policy, address]);

  const emergencyPause = useCallback(async (): Promise<string | null> => {
    if (!isConnected || !addresses.firewall) return null;
    try {
      const hash = await writeContractAsync({
        address: addresses.firewall as `0x${string}`,
        abi: FirewallAbi,
        functionName: 'pause',
      });
      return hash;
    } catch (e) {
      console.error('emergencyPause EVM error:', e);
      return null;
    }
  }, [isConnected, writeContractAsync, addresses.firewall]);

  const emergencyResume = useCallback(async (): Promise<string | null> => {
    if (!isConnected || !addresses.firewall) return null;
    try {
      const hash = await writeContractAsync({
        address: addresses.firewall as `0x${string}`,
        abi: FirewallAbi,
        functionName: 'unpause',
      });
      return hash;
    } catch (e) {
      console.error('emergencyResume EVM error:', e);
      return null;
    }
  }, [isConnected, writeContractAsync, addresses.firewall]);

  const updatePolicy = useCallback(
    async (
      allowedSelectors: string[],
      maxValue: number,
      dailyTxLimit: number,
      cooldownSeconds: number = 0,
    ): Promise<string | null> => {
      if (!isConnected || !addresses.policy || !address) return null;
      try {
        const policy: PolicyStruct = {
          agent: address,
          isActive: true,
          maxValuePerTx: BigInt(maxValue),
          maxGasPerTx: 0n,
          dailyTxLimit: BigInt(dailyTxLimit),
          cooldownSeconds: BigInt(cooldownSeconds),
          allowedTargets: [],
          allowedSelectors: allowedSelectors.map((s) => s as `0x${string}`),
          extraData: '0x',
        };
        const hash = await writeContractAsync({
          address: addresses.policy as `0x${string}`,
          abi: PolicyAbi,
          functionName: 'setPolicy',
          args: [address, policy],
        });
        return hash;
      } catch (e) {
        console.error('updatePolicy EVM error:', e);
        return null;
      }
    },
    [isConnected, writeContractAsync, addresses.policy, address],
  );

  return {
    fetchStats,
    fetchPaused,
    fetchAuditEntries,
    fetchPolicy,
    emergencyPause,
    emergencyResume,
    updatePolicy,
    isConnected,
    address,
  };
}
