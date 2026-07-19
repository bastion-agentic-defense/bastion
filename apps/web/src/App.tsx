import { useMemo } from 'react';
import { BrowserRouter, Routes, Route, Navigate } from 'react-router-dom';
import { ConnectionProvider, WalletProvider } from '@solana/wallet-adapter-react';
import { WalletModalProvider } from '@solana/wallet-adapter-react-ui';
import { PhantomWalletAdapter } from '@solana/wallet-adapter-phantom';
import { SolflareWalletAdapter } from '@solana/wallet-adapter-solflare';
import { BackpackWalletAdapter } from '@solana/wallet-adapter-backpack';
import '@solana/wallet-adapter-react-ui/styles.css';

import { ThemeProvider } from './context/ThemeContext';
import { ChainProvider } from './context/ChainContext';
import { RouteErrorBoundary } from './components/RouteErrorBoundary';
import { SmoothScroll } from './components/SmoothScroll';
import { CHAINS } from './lib/chains';
import Landing from './pages/Landing';
import Docs from './pages/Docs';
import Integrate from './pages/integrate/Integrate';
import Dashboard from './pages/Dashboard';
import AgentList from './pages/AgentList';
import AgentDetail from './pages/AgentDetail';
import DeployAgent from './pages/DeployAgent';

function AppRoutes() {
  return (
    <Routes>
      <Route path="/"              element={<Landing />} />
      <Route path="/integrate"     element={<Integrate />} />
      <Route path="/docs"          element={<Docs />} />
      <Route path="/docs/:slug"    element={<Docs />} />
      <Route path="/dashboard"     element={<Dashboard />} />
      <Route path="/agents"        element={<AgentList />} />
      <Route path="/agents/deploy" element={<DeployAgent />} />
      <Route path="/agents/:id"    element={<AgentDetail />} />
      <Route path="*"              element={<Navigate to="/" replace />} />
    </Routes>
  );
}

/**
 * Solana wallet context.
 *
 * Every page that reads `useWallet()` / `useConnection()` — Integrate, Dashboard,
 * AgentList, DeployAgent and the `useBastionProgram` hook — throws on render if
 * these providers are absent, which blanks the entire route. They are mounted at
 * the root so no route can regress that way again.
 */
function SolanaProviders({ children }: { children: React.ReactNode }) {
  const endpoint = CHAINS.solana.rpcUrl;
  const wallets = useMemo(
    () => [
      new PhantomWalletAdapter(),
      new SolflareWalletAdapter(),
      new BackpackWalletAdapter(),
    ],
    [],
  );

  return (
    <ConnectionProvider endpoint={endpoint}>
      <WalletProvider wallets={wallets} autoConnect>
        <WalletModalProvider>{children}</WalletModalProvider>
      </WalletProvider>
    </ConnectionProvider>
  );
}

export function App() {
  return (
    <ThemeProvider>
      <ChainProvider>
        <SolanaProviders>
          <BrowserRouter>
            <SmoothScroll>
              <RouteErrorBoundary>
                <AppRoutes />
              </RouteErrorBoundary>
            </SmoothScroll>
          </BrowserRouter>
        </SolanaProviders>
      </ChainProvider>
    </ThemeProvider>
  );
}
