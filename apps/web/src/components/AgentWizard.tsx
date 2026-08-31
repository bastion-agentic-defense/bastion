import { useState } from 'react';
import { useAccount } from 'wagmi';
import { useSolanaWallet, SolanaWalletButton } from '../context/SolanaWalletContext';
import { useSidecar } from '../hooks/useSidecar';

type Chain = 'evm' | 'solana';

const CAPABILITIES = [
  { key: 'TRANSFER', label: 'Token Transfer', bit: 1 << 0, icon: '→' },
  { key: 'SWAP', label: 'DEX Swap', bit: 1 << 1, icon: '' },
  { key: 'NFT_MINT', label: 'NFT Mint', bit: 1 << 2, icon: '' },
  { key: 'STAKE', label: 'Staking', bit: 1 << 4, icon: '' },
];

const STEPS = [
  { num: '01', title: 'Install SDK', desc: 'Add the Bastion SDK to your agent project' },
  { num: '02', title: 'Register Agent', desc: 'Create an identity for your AI agent' },
  { num: '03', title: 'Set Policy', desc: 'Configure transaction limits and allowed targets' },
  { num: '04', title: 'Simulate', desc: 'Send a test transaction through the firewall' },
];

export default function AgentWizard() {
  const { address } = useAccount();
  const { publicKey: solanaPublicKey } = useSolanaWallet();
  const sidecar = useSidecar();

  const [chain, setChain] = useState<Chain>('evm');
  const [step, setStep] = useState(0);
  const [agentName, setAgentName] = useState('');
  const [capabilities, setCapabilities] = useState(0);
  const [maxNativePerTx, setMaxNativePerTx] = useState(1);
  const [rateLimit, setRateLimit] = useState(60);
  const [allowedTargets, setAllowedTargets] = useState('');
  const [simTx, setSimTx] = useState('');
  const [solTo, setSolTo] = useState('');
  const [solAmount, setSolAmount] = useState(1000000);
  const [simResult, setSimResult] = useState<string | null>(null);
  const [simLoading, setSimLoading] = useState(false);

  const authorityAddress = chain === 'evm' ? address : solanaPublicKey?.toBase58();

  const installCmd =
    chain === 'evm'
      ? 'pnpm add @zkos-labs/bastion-agentique viem wagmi'
      : 'pnpm add @zkos-labs/bastion-agentique';

  const registerCode =
    chain === 'evm'
      ? `import { BastionEVMClient } from "@zkos-labs/bastion-agentique";

// EVM agent registration via the Bastion contracts.
const client = new BastionEVMClient({ publicClient, walletClient });
// Register through the sidecar: POST /agents with your EVM DID.
const did = \`did:bastion:evm:\${walletClient.account.address}\`;
const res = await fetch(SIDECAR_URL + "/agents", {
  method: "POST",
  headers: { "Content-Type": "application/json" },
  body: JSON.stringify({
    did,
    authority_pubkey: walletClient.account.address,
    name: "${agentName || 'my-agent'}",
  }),
});`
      : `import { BastionSidecar } from "@zkos-labs/bastion-agentique";

// Solana agent registration -- Bastion has no on-chain program on Solana
// (retired in favor of RPC-based simulation); registration is entirely
// through the sidecar.
const sidecar = new BastionSidecar({ baseUrl: SIDECAR_URL });
const did = \`did:bastion:solana:\${pubkey}\`;
const res = await fetch(SIDECAR_URL + "/agents", {
  method: "POST",
  headers: { "Content-Type": "application/json" },
  body: JSON.stringify({
    did,
    authority_pubkey: "${authorityAddress || 'pubkey'}",
    name: "${agentName || 'my-agent'}",
  }),
});`;

  const policyCode =
    chain === 'evm'
      ? `await client.writePolicy(
  walletClient.account.address,
  {
    isActive: true,
    maxValuePerTx: BigInt(${maxNativePerTx}),
    dailyTxLimit: BigInt(${rateLimit}),
    allowedTargets: [${allowedTargets.split('\n')[0]?.trim() ? `"${allowedTargets.split('\n')[0]?.trim()}"` : ''}],
  }
);`
      : `// Solana has no per-agent on-chain policy contract (no Anchor program is
// deployed) -- policy is configured on the sidecar itself and applies to
// every registered agent's simulated transactions.
await sidecar.updatePolicy({
  max_sol_per_tx: ${maxNativePerTx},
  rate_limit_per_minute: ${rateLimit},
  allowed_programs: [${allowedTargets.split('\n')[0]?.trim() ? `"${allowedTargets.split('\n')[0]?.trim()}"` : ''}],
});`;

  const simCode =
    chain === 'evm'
      ? [
          '// Send an EVM transaction through the sidecar',
          "const SIDECAR_URL = import.meta.env?.VITE_SIDECAR_URL || 'https://bastion-agentique.fly.dev';",
          'const response = await fetch(SIDECAR_URL + "/api/v2/simulate-evm", {',
          '  method: "POST",',
          '  headers: { "Content-Type": "application/json" },',
          '  body: JSON.stringify({',
          `    transaction: "${simTx || '0x...'}",`,
          '    intent: "test transaction from Bastion agent wizard"',
          '  })',
          '});',
          'const result = await response.json();',
        ].join('\n')
      : [
          '// Send a Solana operation through the sidecar',
          "const SIDECAR_URL = import.meta.env?.VITE_SIDECAR_URL || 'https://bastion-agentique.fly.dev';",
          'const response = await fetch(SIDECAR_URL + "/api/v2/simulate-solana", {',
          '  method: "POST",',
          '  headers: { "Content-Type": "application/json" },',
          '  body: JSON.stringify({',
          `    to: "${solTo || '<base58 pubkey>'}",`,
          `    amount: ${solAmount},`,
          '    intent: "test transfer from Bastion agent wizard"',
          '  })',
          '});',
          'const result = await response.json();',
        ].join('\n');

  async function handleSimulate() {
    setSimLoading(true);
    setSimResult(null);
    try {
      if (chain === 'evm') {
        const result = await sidecar.simulateEvm(
          { from: authorityAddress || '', to: simTx },
          'wizard test transaction',
        );
        setSimResult(JSON.stringify(result, null, 2));
      } else {
        const result = await sidecar.simulateSolana(
          { to: solTo, amount: solAmount },
          'wizard test transaction',
        );
        setSimResult(JSON.stringify(result, null, 2));
      }
    } catch (e) {
      setSimResult(`Error: ${String(e)}`);
    }
    setSimLoading(false);
  }

  return (
    <div className="space-y-8">
      {/* Chain selector */}
      <div className="flex flex-wrap items-center gap-3">
        <div className="flex rounded-lg overflow-hidden" style={{ border: '1px solid rgba(255,255,255,0.08)' }}>
          {(['evm', 'solana'] as const).map(c => (
            <button
              key={c}
              onClick={() => setChain(c)}
              className="px-4 py-2 font-mono text-xs transition-colors"
              style={{ background: chain === c ? '#fff' : 'transparent', color: chain === c ? '#000' : '#a1a1aa' }}
            >
              {c === 'evm' ? 'EVM' : 'Solana'}
            </button>
          ))}
        </div>
        {chain === 'solana' && <SolanaWalletButton />}
        {chain === 'evm' && authorityAddress && (
          <span className="font-mono text-[11px] text-zinc-500">{authorityAddress.slice(0, 6)}...{authorityAddress.slice(-4)} connected</span>
        )}
      </div>

      {/* Step indicator */}
      <div className="flex gap-2 mb-8">
        {STEPS.map((s, i) => (
          <button
            key={s.num}
            onClick={() => setStep(i)}
            className={`flex-1 rounded-xl p-4 text-left transition-all ${i === step ? 'border-white/20 bg-white/[0.04]' : 'border-white/[0.04] bg-transparent'}`}
            style={{ border: '1px solid', borderColor: i === step ? 'rgba(255,255,255,0.15)' : 'rgba(255,255,255,0.04)', opacity: i > step ? 0.4 : 1 }}
          >
            <span className="font-mono text-[10px] text-zinc-600 block mb-1">{s.num}</span>
            <span className="font-sans text-sm font-medium text-white">{s.title}</span>
            <span className="font-sans text-[11px] text-zinc-500 mt-1 block">{s.desc}</span>
          </button>
        ))}
      </div>

      {/* Step 1: Install SDK */}
      {step === 0 && (
        <div className="space-y-4">
          <p className="font-sans text-sm text-zinc-400">
            Install the Bastion SDK into your agent project. {chain === 'evm' ? 'It wraps the EVM contracts and the sidecar REST API.' : 'For Solana, the SDK talks to the sidecar over HTTP only -- there is no on-chain program.'}
          </p>
          <div className="rounded-xl p-4" style={{ background: '#0a0a0a', border: '1px solid rgba(255,255,255,0.06)' }}>
            <pre className="font-mono text-xs text-zinc-300 overflow-x-auto">{installCmd}</pre>
          </div>
          <p className="font-sans text-[11px] text-zinc-500">
            {chain === 'evm' ? 'After installing, create a BastionEVMClient instance with your wagmi/viem clients.' : 'After installing, create a BastionSidecar instance pointed at your sidecar URL.'}
          </p>
          <button onClick={() => setStep(1)} className="rounded-full bg-white text-black px-8 py-3 text-sm font-medium hover:bg-zinc-200 transition-colors">Next: Register Agent →</button>
        </div>
      )}

      {/* Step 2: Register Agent */}
      {step === 1 && (
        <div className="space-y-4">
          <p className="font-sans text-sm text-zinc-400">Register your AI agent. This creates an identity keyed to your {chain === 'evm' ? 'EVM address' : 'Solana pubkey'} with your agent's name and capability bitmask.</p>
          <input value={agentName} onChange={(e) => setAgentName(e.target.value)} placeholder="Agent name (e.g. trading-bot-42)" className="w-full p-3 rounded-lg font-mono text-sm outline-none" style={{ background: '#0a0a0a', border: '1px solid rgba(255,255,255,0.08)', color: '#fff' }} />
          <div className="flex flex-wrap gap-2">
            {CAPABILITIES.map((c) => (
              <button
                key={c.key}
                onClick={() => setCapabilities((p) => p ^ c.bit)}
                className={`px-3 py-2 rounded-lg font-sans text-xs transition-colors ${capabilities & c.bit ? 'bg-white/10 text-white border-white/20' : 'bg-transparent text-zinc-500 border-white/[0.04]'}`}
                style={{ border: '1px solid', borderColor: capabilities & c.bit ? 'rgba(255,255,255,0.2)' : 'rgba(255,255,255,0.04)' }}
              >
                <span className="mr-1">{c.icon}</span>{c.label}
              </button>
            ))}
          </div>
          <div className="rounded-xl p-4" style={{ background: '#0a0a0a', border: '1px solid rgba(255,255,255,0.06)' }}>
            <pre className="font-mono text-xs text-zinc-300 overflow-x-auto whitespace-pre-wrap">{registerCode}</pre>
          </div>
          <div className="flex gap-3">
            <button onClick={() => setStep(0)} className="rounded-full border border-zinc-700 text-zinc-400 px-6 py-3 text-sm font-medium hover:border-zinc-500 transition-colors">← Back</button>
            <button onClick={() => setStep(2)} className="rounded-full bg-white text-black px-8 py-3 text-sm font-medium hover:bg-zinc-200 transition-colors">Next: Set Policy →</button>
          </div>
        </div>
      )}

      {/* Step 3: Set Policy */}
      {step === 2 && (
        <div className="space-y-4">
          <div className="grid grid-cols-2 gap-3">
            <div>
              <label className="block font-sans text-xs text-zinc-400 mb-1">{chain === 'evm' ? 'Max Native per Tx' : 'Max SOL per Tx'}</label>
              <input type="number" value={maxNativePerTx} onChange={(e) => setMaxNativePerTx(Number(e.target.value))} className="w-full p-3 rounded-lg font-mono text-sm outline-none" style={{ background: '#0a0a0a', border: '1px solid rgba(255,255,255,0.08)', color: '#fff' }} />
            </div>
            <div>
              <label className="block font-sans text-xs text-zinc-400 mb-1">Rate Limit (tx/min)</label>
              <input type="number" value={rateLimit} onChange={(e) => setRateLimit(Number(e.target.value))} className="w-full p-3 rounded-lg font-mono text-sm outline-none" style={{ background: '#0a0a0a', border: '1px solid rgba(255,255,255,0.08)', color: '#fff' }} />
            </div>
          </div>
          <div>
            <label className="block font-sans text-xs text-zinc-400 mb-1">{chain === 'evm' ? 'Allowed Targets (one per line)' : 'Allowed Programs (one per line)'}</label>
            <textarea value={allowedTargets} onChange={(e) => setAllowedTargets(e.target.value)} rows={3} placeholder={chain === 'evm' ? '0x...' : '11111111111111111111111111111111'} className="w-full p-3 rounded-lg font-mono text-sm resize-y outline-none" style={{ background: '#0a0a0a', border: '1px solid rgba(255,255,255,0.08)', color: '#fff' }} />
          </div>
          <div className="rounded-xl p-4" style={{ background: '#0a0a0a', border: '1px solid rgba(255,255,255,0.06)' }}>
            <pre className="font-mono text-xs text-zinc-300 overflow-x-auto whitespace-pre-wrap">{policyCode}</pre>
          </div>
          <div className="flex gap-3">
            <button onClick={() => setStep(1)} className="rounded-full border border-zinc-700 text-zinc-400 px-6 py-3 text-sm font-medium transition-colors">← Back</button>
            <button onClick={() => setStep(3)} className="rounded-full bg-white text-black px-8 py-3 text-sm font-medium hover:bg-zinc-200 transition-colors">Next: Simulate →</button>
          </div>
        </div>
      )}

      {/* Step 4: Simulate */}
      {step === 3 && (
        <div className="space-y-4">
          {chain === 'evm' ? (
            <>
              <p className="font-sans text-sm text-zinc-400">Paste an EVM target address and send a test transaction through the Bastion firewall.</p>
              <textarea value={simTx} onChange={(e) => setSimTx(e.target.value)} rows={2} placeholder="0x... (to address)" className="w-full p-3 rounded-lg font-mono text-sm resize-y outline-none" style={{ background: '#0a0a0a', border: '1px solid rgba(255,255,255,0.08)', color: '#fff' }} />
            </>
          ) : (
            <>
              <p className="font-sans text-sm text-zinc-400">Enter a Solana destination pubkey and amount to send a test transfer through the Bastion firewall.</p>
              <input value={solTo} onChange={(e) => setSolTo(e.target.value)} placeholder="Destination pubkey (base58)" className="w-full p-3 rounded-lg font-mono text-sm outline-none" style={{ background: '#0a0a0a', border: '1px solid rgba(255,255,255,0.08)', color: '#fff' }} />
              <input type="number" value={solAmount} onChange={(e) => setSolAmount(Number(e.target.value))} placeholder="Amount (lamports)" className="w-full p-3 rounded-lg font-mono text-sm outline-none" style={{ background: '#0a0a0a', border: '1px solid rgba(255,255,255,0.08)', color: '#fff' }} />
            </>
          )}
          <button
            onClick={handleSimulate}
            disabled={simLoading || (chain === 'evm' ? !simTx.trim() : !solTo.trim())}
            className="rounded-full bg-white text-black px-8 py-3 text-sm font-medium hover:bg-zinc-200 transition-colors disabled:opacity-50"
          >
            {simLoading ? 'Simulating...' : 'Run Simulation'}
          </button>
          {simResult && (
            <div className="rounded-xl p-4" style={{ background: '#0a0a0a', border: '1px solid rgba(255,255,255,0.06)' }}>
              <pre className="font-mono text-xs text-zinc-300 overflow-x-auto whitespace-pre-wrap max-h-64">{simResult}</pre>
            </div>
          )}
          <div className="rounded-xl p-4" style={{ background: '#0a0a0a', border: '1px solid rgba(255,255,255,0.06)' }}>
            <pre className="font-mono text-xs text-zinc-300 overflow-x-auto whitespace-pre-wrap">{simCode}</pre>
          </div>
          <button onClick={() => setStep(2)} className="rounded-full border border-zinc-700 text-zinc-400 px-6 py-3 text-sm font-medium transition-colors">← Back</button>
        </div>
      )}
    </div>
  );
}
