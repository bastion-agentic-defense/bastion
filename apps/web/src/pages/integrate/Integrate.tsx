import { useNavigate } from 'react-router-dom';
import { useAccount } from 'wagmi';
import { Navbar } from '../../components/Navbar';
import AgentWizard from '../../components/AgentWizard';
import InstallSection from './InstallSection';
import ReputationSection from './ReputationSection';
import QuickStartSection from './QuickStartSection';
import McpSection from './McpSection';
import PricingSection from './PricingSection';
import ChainSupportSection from './ChainSupportSection';
import PersistentSetup from './PersistentSetup';
import ApiReference from './ApiReference';
import LiveTest from './LiveTest';

export default function Integrate() {
  const { isConnected: solConnected } = useAccount();
  const navigate = useNavigate();

  return (
    <div className="relative min-h-screen w-full overflow-x-hidden" style={{ background: 'var(--bg)' }}>
      <Navbar />

      <main className="relative z-10 max-w-6xl mx-auto px-6 pt-32 pb-20">
        {/* Hero */}
        <section className="text-center mb-20" aria-labelledby="integrate-headline">
          <h1
            id="integrate-headline"
            className="animate-fade-rise font-display max-w-3xl mx-auto"
            style={{
              fontSize: 'clamp(1.95rem, 4.4vw, 3.1rem)',
              lineHeight: '1.1',
              letterSpacing: '-0.032em',
              fontWeight: 500,
              color: 'var(--text-primary)',
            }}
          >
            One line to secure{' '}
            <span style={{ color: 'var(--accent-text)' }}>your agent</span>.
          </h1>

          <p
            className="animate-fade-rise-delay font-sans mt-6 max-w-xl mx-auto text-base leading-relaxed"
            style={{ color: 'var(--text-muted)' }}
          >
            Install the SDK, register your agent, set a policy, connect via MCP. Every transaction validated before signing. Every API call inspected before sending. Multi-chain native. Web2 proxy. Zero trust.

            Bastion is in alpha testing. Use with caution in production environments.
          </p>

          {/* Action buttons */}
          <div className="flex items-center justify-center gap-4 mt-10">
            {solConnected ? (
              <button
                onClick={() => navigate('/dashboard')}
                className="rounded-full px-8 py-3 text-sm font-medium font-sans transition-transform duration-150 hover:scale-[1.03] active:scale-[0.98] focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-[var(--accent)] focus-visible:ring-offset-2"
                style={{ background: 'var(--text-primary)', color: 'var(--bg)' }}
              >
                Go to Dashboard
              </button>
            ) : (
              <a
                href="#install"
                className="rounded-full px-8 py-3 text-sm font-medium font-sans transition-transform duration-150 hover:scale-[1.03] active:scale-[0.98] focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-[var(--accent)] focus-visible:ring-offset-2"
                style={{ background: 'var(--text-primary)', color: 'var(--bg)', textDecoration: 'none' }}
              >
                Start Integrating
              </a>
            )}
            <a
              href="https://github.com/zkos-labs/bastion"
              target="_blank"
              rel="noopener noreferrer"
              className="font-sans nav-link no-underline"
              style={{ fontSize: '14px', color: 'var(--text-muted)', textDecoration: 'none' }}
            >
              Source ↗
            </a>
          </div>

          {/* EVM Status */}
          <div
            className="animate-fade-rise-delay-2 mt-10 mx-auto max-w-sm rounded-xl p-5"
            style={{ background: 'var(--bg-subtle)', border: '1px solid var(--card-border)' }}
          >
            <div className="flex items-center gap-3 mb-2">
              <span className="font-sans text-sm font-medium" style={{ color: 'var(--text-primary)' }}>
EVM (Celo / Base / Sepolia) · Solana (RPC simulation)
            </span>
              <span
                className="font-mono text-[9px] px-2 py-0.5 rounded-full ml-auto"
                style={{ background: 'rgba(234,179,8,0.1)', color: '#eab308', border: '1px solid rgba(234,179,8,0.25)' }}
              >
                Live Testnet
              </span>
            </div>
            <p className="font-sans text-xs" style={{ color: 'var(--text-muted)' }}>
              EVM contracts deployed on Sepolia. ERC-7579 validator module and EIP-712 audit trail operational across Ethereum, Base, and Celo.
            </p>
          </div>
        </section>

        {/* Sections */}
        <div className="space-y-20 pb-20" id="install">
          <InstallSection />

          {/* Guided Setup: interactive walkthrough, EVM or Solana */}
          <section aria-labelledby="guided-setup-heading">
            <h3
              id="guided-setup-heading"
              className="font-sans text-sm uppercase tracking-wider mb-4"
              style={{ color: 'var(--text-muted)' }}
            >
              Guided Setup
            </h3>
            <div
              className="rounded-xl p-6"
              style={{ background: 'var(--card-bg)', border: '1px solid var(--card-border)' }}
            >
              <AgentWizard />
            </div>
          </section>

          <QuickStartSection />
          <ReputationSection />
          <McpSection />
          <PricingSection />
          <PersistentSetup />
          <ApiReference />
          <ChainSupportSection />
          <LiveTest />
        </div>
      </main>
    </div>
  );
}
