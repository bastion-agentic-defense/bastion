import { getDefaultConfig } from '@rainbow-me/rainbowkit';
import { http } from 'wagmi';
import { defineChain } from 'viem';

const sepolia = defineChain({
  id: 11_155_111,
  name: 'Ethereum Sepolia',
  nativeCurrency: { name: 'Sepolia Ether', symbol: 'ETH', decimals: 18 },
  rpcUrls: {
    default: {
      http: [import.meta.env.VITE_EVM_RPC_URL || 'https://ethereum-sepolia-rpc.publicnode.com'],
    },
  },
  blockExplorers: {
    default: {
      name: 'Etherscan',
      url: import.meta.env.VITE_EVM_EXPLORER_URL || 'https://sepolia.etherscan.io',
    },
  },
});

const base = defineChain({
  id: 8_453,
  name: 'Base',
  nativeCurrency: { name: 'Ether', symbol: 'ETH', decimals: 18 },
  rpcUrls: {
    default: { http: [import.meta.env.VITE_BASE_RPC_URL || 'https://mainnet.base.org'] },
  },
  blockExplorers: {
    default: { name: 'Basescan', url: 'https://basescan.org' },
  },
});

const celo = defineChain({
  id: 42_220,
  name: 'Celo',
  nativeCurrency: { name: 'CELO', symbol: 'CELO', decimals: 18 },
  rpcUrls: {
    default: { http: [import.meta.env.VITE_CELO_RPC_URL || 'https://forno.celo.org'] },
  },
  blockExplorers: {
    default: { name: 'Celoscan', url: 'https://celoscan.io' },
  },
});

const zksync = defineChain({
  id: 324,
  name: 'zkSync Era',
  nativeCurrency: { name: 'Ether', symbol: 'ETH', decimals: 18 },
  rpcUrls: {
    default: {
      http: [import.meta.env.VITE_ZKSYNC_RPC_URL || 'https://mainnet.era.zksync.io'],
    },
  },
  blockExplorers: {
    default: { name: 'zkSync Explorer', url: 'https://explorer.zksync.io' },
  },
});

const robinhood = defineChain({
  id: 4_663,
  name: 'Robinhood Chain',
  nativeCurrency: { name: 'Ether', symbol: 'ETH', decimals: 18 },
  rpcUrls: {
    default: {
      http: [
        import.meta.env.VITE_ROBINHOOD_RPC_URL || 'https://rpc.mainnet.chain.robinhood.com',
      ],
    },
  },
  blockExplorers: {
    default: {
      name: 'Blockscout',
      url: 'https://robinhoodchain.blockscout.com',
    },
  },
});

export const config = getDefaultConfig({
  appName: 'Bastion',
  projectId:
    import.meta.env.VITE_WALLETCONNECT_PROJECT_ID ||
    // Placeholder — replace with your project ID from https://cloud.walletconnect.com
    '00000000000000000000000000000000',
  chains: [sepolia, base, celo, zksync, robinhood],
  transports: {
    [sepolia.id]: http(sepolia.rpcUrls.default.http[0]),
    [base.id]: http(base.rpcUrls.default.http[0]),
    [celo.id]: http(celo.rpcUrls.default.http[0]),
    [zksync.id]: http(zksync.rpcUrls.default.http[0]),
    [robinhood.id]: http(robinhood.rpcUrls.default.http[0]),
  },
  ssr: false,
});
