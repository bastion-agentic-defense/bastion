import { useState } from 'react';

const EVM_INSTALL = `npm install @zkos-labs/bastion-sdk viem wagmi`;
const EVM_DEPS = `# or
pnpm add @zkos-labs/bastion-sdk viem wagmi
# or
yarn add @zkos-labs/bastion-sdk viem wagmi`;

const WEB2_INSTALL = `npm install @zkos-labs/web2-sdk`;
const WEB2_DEPS = `# or
pnpm add @zkos-labs/web2-sdk
# or
yarn add @zkos-labs/web2-sdk`;

export default function InstallSection() {
  const [copied, setCopied] = useState(false);

  function handleCopy() {
    navigator.clipboard.writeText(EVM_INSTALL);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  }

  return (
    <section className="max-w-3xl mx-auto" aria-labelledby="install-heading">
      <h3
        id="install-heading"
        className="font-sans text-sm uppercase tracking-wider mb-4"
        style={{ color: 'var(--text-muted)' }}
      >
        Step 1: Install
      </h3>

      <div
        className="rounded-xl overflow-hidden mb-4"
        style={{ background: 'var(--card-bg)', border: '1px solid var(--card-border)' }}
      >
        <div className="flex items-center justify-between px-4 py-2" style={{ borderBottom: '1px solid var(--border)' }}>
          <span className="font-mono text-xs" style={{ color: 'var(--text-muted)' }}>EVM - Terminal</span>
          <button
            onClick={handleCopy}
            className="font-sans text-xs font-medium transition-colors duration-150 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-[var(--accent)] rounded px-2 py-0.5"
            style={{ color: copied ? '#22c55e' : 'var(--text-muted)' }}
          >
            {copied ? 'Copied' : 'Copy'}
          </button>
        </div>
        <pre className="p-4 overflow-x-auto">
          <code className="font-mono text-sm leading-relaxed block" style={{ color: 'var(--text-primary)' }}>
            {EVM_INSTALL}
          </code>
        </pre>
        <pre className="px-4 pb-4 overflow-x-auto">
          <code className="font-mono text-sm leading-relaxed block" style={{ color: 'var(--text-muted)' }}>
            {EVM_DEPS}
          </code>
        </pre>
      </div>

      {/* Web2 Proxy SDK */}
      <div
        className="rounded-xl overflow-hidden mb-4"
        style={{ background: 'var(--card-bg)', border: '1px solid var(--card-border)' }}
      >
        <div className="flex items-center justify-between px-4 py-2" style={{ borderBottom: '1px solid var(--border)' }}>
          <span className="font-mono text-xs" style={{ color: 'var(--text-muted)' }}>Web2 - Terminal</span>
          <button
            onClick={() => {
              navigator.clipboard.writeText(WEB2_INSTALL);
              setCopied(true);
              setTimeout(() => setCopied(false), 2000);
            }}
            className="font-sans text-xs font-medium transition-colors duration-150 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-[var(--accent)] rounded px-2 py-0.5"
            style={{ color: copied ? '#22c55e' : 'var(--text-muted)' }}
          >
            {copied ? 'Copied' : 'Copy'}
          </button>
        </div>
        <pre className="p-4 overflow-x-auto">
          <code className="font-mono text-sm leading-relaxed block" style={{ color: 'var(--text-primary)' }}>
            {WEB2_INSTALL}
          </code>
        </pre>
        <pre className="px-4 pb-4 overflow-x-auto">
          <code className="font-mono text-sm leading-relaxed block" style={{ color: 'var(--text-muted)' }}>
            {WEB2_DEPS}
          </code>
        </pre>
      </div>

      {/* Chain support note */}
      <div
        className="rounded-xl p-4"
        style={{ background: 'var(--bg-subtle)', border: '1px solid var(--border)' }}
      >
        <div className="flex items-center gap-2 mb-1">
          <span className="font-mono text-[10px] px-2 py-0.5 rounded-full" style={{ background: 'rgba(59,130,246,0.1)', color: '#3B82F6' }}>EVM</span>
          <span className="font-sans text-xs" style={{ color: 'var(--text-muted)' }}>Supports Ethereum, Base, Celo, zkSync, Robinhood and Monad</span>
        </div>
        <code className="font-mono text-xs" style={{ color: 'var(--text-muted)' }}>
          # ERC-7579 validator module · EIP-712 audit trail · ERC-8004 agent identity
        </code>
      </div>
    </section>
  );
}
