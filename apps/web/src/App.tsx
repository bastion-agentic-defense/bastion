import { BrowserRouter, Routes, Route, Navigate } from 'react-router-dom';
import { WagmiProvider } from 'wagmi';
import { RainbowKitProvider } from '@rainbow-me/rainbowkit';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import '@rainbow-me/rainbowkit/styles.css';

import { ThemeProvider } from './context/ThemeContext';
import { SolanaWalletContextProvider } from './context/SolanaWalletContext';
import { RouteErrorBoundary } from './components/RouteErrorBoundary';
import { SmoothScroll } from './components/SmoothScroll';
import { config } from './lib/evmConfig';
import Landing from './pages/Landing';
import Docs from './pages/Docs';
import Integrate from './pages/integrate/Integrate';
import Dashboard from './pages/Dashboard';
import AgentList from './pages/AgentList';
import AgentDetail from './pages/AgentDetail';
import DeployAgent from './pages/DeployAgent';

const queryClient = new QueryClient();

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

export function App() {
  return (
    <WagmiProvider config={config}>
      <QueryClientProvider client={queryClient}>
        <RainbowKitProvider>
          <ThemeProvider>
            <SolanaWalletContextProvider>
              <BrowserRouter>
                <SmoothScroll>
                  <RouteErrorBoundary>
                    <AppRoutes />
                  </RouteErrorBoundary>
                </SmoothScroll>
              </BrowserRouter>
            </SolanaWalletContextProvider>
          </ThemeProvider>
        </RainbowKitProvider>
      </QueryClientProvider>
    </WagmiProvider>
  );
}
