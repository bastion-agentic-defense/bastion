import { useMemo, type ReactNode } from 'react';
import { ConnectionProvider, WalletProvider } from '@solana/wallet-adapter-react';
import { WalletModalProvider, WalletMultiButton } from '@solana/wallet-adapter-react-ui';
import '@solana/wallet-adapter-react-ui/styles.css';

// No explicit adapters are listed in `wallets` below: modern Solana wallets
// (Phantom, Solflare, Backpack, ...) self-register via the Wallet Standard
// and are auto-detected by @solana/wallet-adapter-react. This intentionally
// avoids depending on the monolithic, deprecated @solana/wallet-adapter-wallets
// package, which pulls in every legacy adapter whether installed or not.
const SOLANA_RPC_URL =
  import.meta.env.VITE_SOLANA_RPC_URL || 'https://api.devnet.solana.com';

export function SolanaWalletContextProvider({ children }: { children: ReactNode }) {
  const wallets = useMemo(() => [], []);

  return (
    <ConnectionProvider endpoint={SOLANA_RPC_URL}>
      <WalletProvider wallets={wallets} autoConnect>
        <WalletModalProvider>{children}</WalletModalProvider>
      </WalletProvider>
    </ConnectionProvider>
  );
}

export { WalletMultiButton as SolanaWalletButton };
export { useWallet as useSolanaWallet, useConnection as useSolanaConnection } from '@solana/wallet-adapter-react';
