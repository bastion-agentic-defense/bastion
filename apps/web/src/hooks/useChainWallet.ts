import { useChain } from '../context/ChainContext';
import { useWallet } from '@solana/wallet-adapter-react';
import { useAccount, useConnect, useDisconnect } from 'wagmi';

export interface ChainWalletState {
  connected: boolean;
  address: string | null;
  chain: string;
}

export function useChainWallet(): ChainWalletState {
  const { chain } = useChain();

  const solWallet = useWallet();
  const { address: evmAddress, isConnected: evmConnected } = useAccount();

  if (chain === 'solana') {
    return {
      connected: solWallet.connected,
      address: solWallet.publicKey?.toBase58() ?? null,
      chain: 'solana',
    };
  }

  return {
    connected: evmConnected,
    address: evmAddress ?? null,
    chain: 'evm',
  };
}
