const SOLANA_SUPPORT = {
  wallets: ['Phantom', 'Solflare', 'Backpack'],
  rpc: 'Helius / QuickNode / Triton',
  features: [
    'Full TypeScript SDK (@zkos-labs/sdk)',
    'On-chain audit via Anchor PDA',
    'Helius simulation integration',
    'Program allowlists',
    'Native token caps',
    'Circuit breaker',
  ],
  status: 'Alpha',
} as const;

const EVM_SUPPORT = {
  wallets: ['MetaMask', 'Rainbow', 'WalletConnect'],
  rpc: 'Celo, Base, Polygon RPCs',
  features: [
    'ERC-7579 validator module',
    'EIP-712 immutable audit trail',
    'ERC-8004 agent identity',
    'Solidity firewall (Foundry)',
    'Per-agent policy engine',
  ],
  status: 'In Progress',
} as const;

const WEB2_SUPPORT = {
  providers: ['OpenAI', 'Stripe', 'Slack', 'GitHub', 'AWS'],
  integration: 'Proxy, SDK, Ingest',
  features: [
    'Full TypeScript SDK (@zkos-labs/web2-sdk)',
    'HTTP forward proxy firewall',
    'OpenAPI spec auto-configuration',
    'Provider budget enforcement',
    'Content inspection for PII and secrets',
  ],
  status: 'In Progress',
} as const;

export default function ChainSupportSection() {
  return (
    <section className="max-w-3xl mx-auto" aria-labelledby="chains-heading">
      <h3
        id="chains-heading"
        className="font-sans text-sm uppercase tracking-wider mb-4"
        style={{ color: 'var(--text-muted)' }}
      >
        Chain Support
      </h3>

      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
        {/* Solana */}
        <div
          className="rounded-xl p-6"
          style={{ background: 'var(--card-bg)', border: '1px solid var(--card-border)' }}
        >
          <div className="flex items-center gap-3 mb-4">
            <span style={{ color: '#9945FF', fontSize: '1.5em' }}></span>
            <div>
              <h4 className="font-sans font-semibold text-base" style={{ color: 'var(--text-primary)' }}>
                Solana
              </h4>
              <span
                className="inline-block font-mono text-xs px-2 py-0.5 rounded-full mt-0.5"
                style={{ background: 'rgba(34,197,94,0.1)', color: '#22c55e', border: '1px solid rgba(34,197,94,0.2)' }}
              >
                {SOLANA_SUPPORT.status}
              </span>
            </div>
          </div>

          <div className="space-y-3">
            <div>
              <span className="font-sans text-xs font-medium" style={{ color: 'var(--text-muted)' }}>Wallets</span>
              <p className="font-sans text-sm mt-1" style={{ color: 'var(--text-primary)' }}>{SOLANA_SUPPORT.wallets.join(', ')}</p>
            </div>
            <div>
              <span className="font-sans text-xs font-medium" style={{ color: 'var(--text-muted)' }}>RPC</span>
              <p className="font-sans text-sm mt-1" style={{ color: 'var(--text-primary)' }}>{SOLANA_SUPPORT.rpc}</p>
            </div>
            <div>
              <span className="font-sans text-xs font-medium" style={{ color: 'var(--text-muted)' }}>Features</span>
              <ul className="mt-2 space-y-1">
                {SOLANA_SUPPORT.features.map((f) => (
                  <li key={f} className="font-sans text-sm flex items-center gap-2" style={{ color: 'var(--text-primary)' }}>
                    <span style={{ color: 'var(--accent)' }}>+</span>
                    {f}
                  </li>
                ))}
              </ul>
            </div>
          </div>
        </div>

        {/* EVM */}
        <div
          className="rounded-xl p-6"
          style={{ background: 'var(--card-bg)', border: '1px solid var(--card-border)' }}
        >
          <div className="flex items-center gap-3 mb-4">
            <span style={{ color: '#3B82F6', fontSize: '1.5em' }}></span>
            <div>
              <h4 className="font-sans font-semibold text-base" style={{ color: 'var(--text-primary)' }}>
                EVM
              </h4>
              <span
                className="inline-block font-mono text-xs px-2 py-0.5 rounded-full mt-0.5"
                style={{ background: 'rgba(234,179,8,0.1)', color: '#eab308', border: '1px solid rgba(234,179,8,0.25)' }}
              >
                {EVM_SUPPORT.status}
              </span>
            </div>
          </div>

          <div className="space-y-3">
            <div>
              <span className="font-sans text-xs font-medium" style={{ color: 'var(--text-muted)' }}>Wallets</span>
              <p className="font-sans text-sm mt-1" style={{ color: 'var(--text-primary)' }}>{EVM_SUPPORT.wallets.join(', ')}</p>
            </div>
            <div>
              <span className="font-sans text-xs font-medium" style={{ color: 'var(--text-muted)' }}>RPC</span>
              <p className="font-sans text-sm mt-1" style={{ color: 'var(--text-primary)' }}>{EVM_SUPPORT.rpc}</p>
            </div>
            <div>
              <span className="font-sans text-xs font-medium" style={{ color: 'var(--text-muted)' }}>Features</span>
              <ul className="mt-2 space-y-1">
                {EVM_SUPPORT.features.map((f) => (
                  <li key={f} className="font-sans text-sm flex items-center gap-2" style={{ color: 'var(--text-primary)' }}>
                    <span style={{ color: 'var(--accent)' }}>+</span>
                    {f}
                  </li>
                ))}
              </ul>
            </div>
          </div>
        </div>

        {/* Web2 Proxy */}
        <div
          className="rounded-xl p-6"
          style={{ background: 'var(--card-bg)', border: '1px solid var(--card-border)' }}
        >
          <div className="flex items-center gap-3 mb-4">
            <div>
              <h4 className="font-sans font-semibold text-base" style={{ color: 'var(--text-primary)' }}>
                Web2 Proxy
              </h4>
              <span
                className="inline-block font-mono text-xs px-2 py-0.5 rounded-full mt-0.5"
                style={{ background: 'rgba(234,179,8,0.1)', color: '#eab308', border: '1px solid rgba(234,179,8,0.25)' }}
              >
                {WEB2_SUPPORT.status}
              </span>
            </div>
          </div>

          <div className="space-y-3">
            <div>
              <span className="font-sans text-xs font-medium" style={{ color: 'var(--text-muted)' }}>Providers</span>
              <p className="font-sans text-sm mt-1" style={{ color: 'var(--text-primary)' }}>{WEB2_SUPPORT.providers.join(', ')}</p>
            </div>
            <div>
              <span className="font-sans text-xs font-medium" style={{ color: 'var(--text-muted)' }}>Integration</span>
              <p className="font-sans text-sm mt-1" style={{ color: 'var(--text-primary)' }}>{WEB2_SUPPORT.integration}</p>
            </div>
            <div>
              <span className="font-sans text-xs font-medium" style={{ color: 'var(--text-muted)' }}>Features</span>
              <ul className="mt-2 space-y-1">
                {WEB2_SUPPORT.features.map((f) => (
                  <li key={f} className="font-sans text-sm flex items-center gap-2" style={{ color: 'var(--text-primary)' }}>
                    <span style={{ color: 'var(--accent)' }}>+</span>
                    {f}
                  </li>
                ))}
              </ul>
            </div>
          </div>
        </div>
      </div>
    </section>
  );
}
