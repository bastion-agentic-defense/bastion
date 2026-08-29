import { useChain } from '../context/ChainContext';
import { useAccount } from 'wagmi';

export interface ChainWalletState {
  connected: boolean;
  address: string | null;
  chain: string;
}

export function useChainWallet(): ChainWalletState {
  const { chain } = useChain();
  const { address: evmAddress, isConnected: evmConnected } = useAccount();

  return {
    connected: evmConnected,
    address: evmAddress ?? null,
    chain,
  };
}
