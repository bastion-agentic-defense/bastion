export type ChainId = 'evm' | 'solana';

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
  evm: {
    id: 'evm',
    name: 'EVM (Ethereum / Base / Celo / zkSync / Robinhood / Monad / Polygon / Arbitrum)',
    shortName: 'EVM',
    icon: '',
    color: '#627EEA',
    rpcUrl: import.meta.env.VITE_EVM_RPC_URL || 'https://ethereum-sepolia-rpc.publicnode.com',
    explorerUrl: import.meta.env.VITE_EVM_EXPLORER_URL || 'https://sepolia.etherscan.io',
  },
  solana: {
    id: 'solana',
    name: 'Solana (RPC simulation)',
    shortName: 'Solana',
    icon: '',
    color: '#9945FF',
    rpcUrl: import.meta.env.VITE_SOLANA_RPC_URL || '',
    explorerUrl: import.meta.env.VITE_SOLANA_EXPLORER_URL || 'https://solscan.io',
  },
};

export const CHAIN_LIST: ChainConfig[] = Object.values(CHAINS);
export const DEFAULT_CHAIN: ChainId = 'evm';
