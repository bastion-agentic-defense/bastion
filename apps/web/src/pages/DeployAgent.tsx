import { useState } from 'react';
import { useNavigate, Link } from 'react-router-dom';
import { useAccount } from 'wagmi';
import { SolanaWalletButton, useSolanaWallet } from '../context/SolanaWalletContext';

const TEMPLATES = [
  {
    id: 'defi',
    name: 'DeFi Agent',
    icon: '',
    color: '#22c55e',
    desc: 'Swap and transfer with policy-gated limits. Uniswap + ERC-20 + native token.',
    caps: 'TRANSFER + SWAP',
    maxNativePerTx: 0.1,
    rateLimit: 10,
  },
  {
    id: 'trading',
    name: 'Trading Agent',
    icon: '',
    color: '#f59e0b',
    desc: 'High-frequency trading with relaxed limits. Higher rate limit for active strategies.',
    caps: 'TRANSFER + SWAP',
    maxNativePerTx: 0.5,
    rateLimit: 50,
  },
  {
    id: 'transfer',
    name: 'Transfer Agent',
    icon: '→',
    color: '#3b82f6',
    desc: 'Simple native token transfer agent. Tight policy, minimal risk.',
    caps: 'TRANSFER only',
    maxNativePerTx: 0.01,
    rateLimit: 5,
  },
  {
    id: 'custom',
    name: 'Custom Agent',
    icon: '✦',
    color: '#a855f7',
    desc: 'Fully customizable. Set your own capabilities, policy, and limits.',
    caps: 'Full control',
    maxNativePerTx: 0.1,
    rateLimit: 10,
  },
];

export default function DeployAgent() {
  const { address, isConnected: connected } = useAccount();
  const { publicKey: solanaPublicKey, connected: solanaConnected } = useSolanaWallet();
  const navigate = useNavigate();
  const [chain, setChain] = useState<'evm' | 'solana'>('evm');
  const [selected, setSelected] = useState<string | null>(null);
  const [agentName, setAgentName] = useState('');
  const [deploying, setDeploying] = useState(false);
  const [result, setResult] = useState<string | null>(null);

  const tpl = TEMPLATES.find(t => t.id === selected);
  const authorityPubkey = chain === 'evm' ? address : solanaPublicKey?.toBase58();
  const canDeploy = chain === 'evm' ? connected && !!address : solanaConnected && !!solanaPublicKey;

  async function handleDeploy() {
    if (!tpl || !canDeploy || !authorityPubkey) return;
    setDeploying(true);
    setResult(null);
    try {
      // Register the agent via the sidecar (POST /agents).
      const res = await fetch(`${import.meta.env.VITE_SIDECAR_URL || 'https://bastion-agentique.fly.dev/'}/agents`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'X-Api-Key': import.meta.env.VITE_BASTION_API_KEY || '',
        },
        body: JSON.stringify({
          did: `did:bastion:${chain}:${authorityPubkey}`,
          authority_pubkey: authorityPubkey,
          sidecar_endpoint: null,
          name: agentName || tpl.name,
        }),
      });
      if (!res.ok) {
        const err = await res.json().catch(() => ({}));
        throw new Error((err as any).error || 'Registration failed');
      }
      setResult(`Agent "${agentName || tpl.name}" registered successfully. Navigate to Dashboard to configure policy.`);
      setTimeout(() => navigate('/dashboard'), 3000);
    } catch (e) {
      setResult(`Error: ${(e as Error).message}`);
    } finally {
      setDeploying(false);
    }
  }

  return (
    <div className="min-h-screen bg-black text-white font-sans">
      <nav className="fixed top-0 inset-x-0 z-50 flex items-center justify-between px-6 py-4 bg-black/80 backdrop-blur-md border-b border-white/[0.06]">
        <Link to="/dashboard" className="font-serif text-lg tracking-tight no-underline text-white">Bastion<span className="text-[8px] align-super ml-px">&reg;</span></Link>
        <Link to="/integrate" className="font-sans text-xs text-blue-400 hover:underline">Integration Guide</Link>
      </nav>

      <main className="pt-32 px-6 pb-8 max-w-4xl mx-auto">
        <h1 className="font-serif text-2xl mb-2">Deploy an Agent</h1>
        <p className="font-sans text-xs text-zinc-500 mb-6">
          Choose a pre-configured template. Bastion sets up the policy and capabilities - your agent is ready in seconds.
        </p>

        {/* Chain toggle + wallet connect */}
        <div className="flex flex-wrap items-center gap-3 mb-8">
          <div className="flex rounded-lg overflow-hidden" style={{ border: '1px solid rgba(255,255,255,0.08)' }}>
            {(['evm', 'solana'] as const).map(c => (
              <button
                key={c}
                onClick={() => setChain(c)}
                className="px-4 py-2 font-mono text-xs transition-colors"
                style={{
                  background: chain === c ? '#fff' : 'transparent',
                  color: chain === c ? '#000' : '#a1a1aa',
                }}
              >
                {c === 'evm' ? 'EVM' : 'Solana'}
              </button>
            ))}
          </div>
          {chain === 'evm' ? (
            connected ? (
              <span className="font-mono text-[11px] text-zinc-500">{address?.slice(0, 6)}...{address?.slice(-4)} connected</span>
            ) : (
              <span className="font-mono text-[11px] text-red-400">Connect an EVM wallet (top of dashboard) to deploy</span>
            )
          ) : (
            <SolanaWalletButton />
          )}
        </div>

        {/* Template grid */}
        <div className="grid grid-cols-1 sm:grid-cols-2 gap-4 mb-8">
          {TEMPLATES.map(t => (
            <button
              key={t.id}
              onClick={() => setSelected(t.id)}
              className="rounded-xl p-5 text-left transition-all duration-300"
              style={{
                background: selected === t.id ? 'rgba(255,255,255,0.06)' : 'rgba(255,255,255,0.02)',
                border: selected === t.id ? `1px solid ${t.color}` : '1px solid rgba(255,255,255,0.06)',
              }}
            >
              <div className="flex items-start justify-between mb-3">
                <span style={{ color: t.color, fontSize: '1.5em' }}>{t.icon}</span>
                <span className="font-mono text-[8px] px-1.5 py-0.5 rounded" style={{ background: `${t.color}20`, color: t.color }}>
                  {t.caps}
                </span>
              </div>
              <h3 className="font-sans text-sm font-medium mb-2">{t.name}</h3>
              <p className="font-sans text-[11px] text-zinc-400 mb-3">{t.desc}</p>
              <div className="space-y-1 font-mono text-[10px] text-zinc-600">
                <div className="flex justify-between">
                  <span>Max native/tx</span>
                  <span className="text-zinc-400">{t.maxNativePerTx}</span>
                </div>
                <div className="flex justify-between">
                  <span>Rate limit</span>
                  <span className="text-zinc-400">{t.rateLimit}/min</span>
                </div>
              </div>
            </button>
          ))}
        </div>

        {/* Agent name + deploy */}
        {selected && (
          <div className="rounded-xl p-6 mb-8" style={{ background: '#0a0a0a', border: '1px solid rgba(255,255,255,0.06)' }}>
            <p className="font-sans text-sm font-medium mb-4">{tpl?.name} Configuration</p>
            <div className="flex gap-3 mb-4">
              <input
                value={agentName}
                onChange={e => setAgentName(e.target.value)}
                placeholder={`Agent name (default: ${tpl?.name})`}
                className="flex-1 p-2.5 rounded-lg font-mono text-sm outline-none"
                style={{ background: '#111', border: '1px solid rgba(255,255,255,0.08)', color: '#fff' }}
              />
              <button
                onClick={handleDeploy}
                disabled={deploying || !canDeploy}
                className="px-6 py-2.5 rounded-lg font-mono text-sm font-medium transition-colors disabled:opacity-50"
                style={{ background: tpl?.color, color: '#000' }}
              >
                {deploying ? 'Deploying...' : 'Deploy Agent'}
              </button>
            </div>
            {!canDeploy && (
              <p className="font-sans text-[10px] text-red-400">
                Connect your {chain === 'evm' ? 'EVM' : 'Solana'} wallet to deploy.
              </p>
            )}
            {result && (
              <div
                className="p-3 rounded font-mono text-xs"
                style={{ background: result.startsWith('Error') ? 'rgba(239,68,68,0.1)' : 'rgba(34,197,94,0.1)', color: result.startsWith('Error') ? '#ef4444' : '#22c55e' }}
              >
                {result}
              </div>
            )}
          </div>
        )}

        {/* Template files */}
        <div className="rounded-xl p-5" style={{ background: '#0a0a0a', border: '1px solid rgba(255,255,255,0.04)' }}>
          <p className="font-sans text-[10px] uppercase tracking-wider text-zinc-500 mb-3">Template Files</p>
          <p className="font-sans text-xs text-zinc-500 mb-2">
            Templates are stored as YAML in <code className="text-zinc-400">apps/web/public/agent-templates/</code>
          </p>
          <div className="space-y-1 font-mono text-[10px] text-zinc-600">
            <div>• defi-agent.yml - DeFi operations (Uniswap + ERC-20)</div>
            <div>• trading-agent.yml - High-frequency (relaxed limits)</div>
            <div>• transfer-agent.yml - Simple native transfer (low risk)</div>
            <div>• custom-agent.yml - Full customization</div>
          </div>
        </div>
      </main>
    </div>
  );
}
