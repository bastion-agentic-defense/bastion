export type ChainId = 'solana' | 'evm';

export interface ChainConfig {
  id: ChainId;
  name: string;
  shortName: string;
  icon: string;
  color: string;
  rpcUrl: string;
  explorerUrl: string;
}

export const CHAINS: Record<ChainId, ChainConfig> = {
  solana: {
    id: 'solana',
    name: 'Solana',
    shortName: 'SOL',
    icon: '◎',
    color: '#9945FF',
    rpcUrl: 'https://api.devnet.solana.com',
    explorerUrl: 'https://explorer.solana.com',
  },
  evm: {
    id: 'evm',
    name: 'EVM (Ethereum/Base/Celo)',
    shortName: 'EVM',
    icon: '⟠',
    color: '#627EEA',
    // Sepolia testnet by default (see docs/EVM_READINESS.md — testnet-only pre-audit).
    rpcUrl: import.meta.env.VITE_EVM_RPC_URL || 'https://ethereum-sepolia-rpc.publicnode.com',
    explorerUrl: import.meta.env.VITE_EVM_EXPLORER_URL || 'https://sepolia.etherscan.io',
  },
};

export const CHAIN_LIST: ChainConfig[] = Object.values(CHAINS);
export const DEFAULT_CHAIN: ChainId = 'solana';
