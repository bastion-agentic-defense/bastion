import { useState, useEffect, useCallback } from 'react';
import { useBastionEVM, type AuditEntryData, type StatsData } from '../hooks/useBastionEVM';

const DECISION_COLORS: Record<string, string> = { ALLOWED: '#22c55e', BLOCKED: '#ef4444', PENDING: '#f59e0b' };

const CARD = { background: '#0a0a0a', border: '1px solid rgba(255,255,255,0.06)' } as const;

// Deployed contract addresses come from Vite env (set after the Sepolia deploy —
// see docs/EVM_READINESS.md §7). Absent → "not deployed" state, never a crash.
const AUDIT_ADDRESS = import.meta.env.VITE_BASTION_AUDIT_ADDRESS as string | undefined;
const EXPLORER_URL = import.meta.env.VITE_EVM_EXPLORER_URL || 'https://sepolia.etherscan.io';

/**
 * Read-only EVM status panel. Surfaces live on-chain data across Sepolia, Base, and Celo.
 * state from the deployed Bastion contracts via the wagmi/viem `useBastionEVM`
 * hook: audit entry count, firewall pause state, and recent AuditRecorded events.
 * Writes (pause / setPolicy) are intentionally omitted from this pass.
 */
export default function EvmStatusPanel() {
  const { fetchStats, fetchPaused, fetchAuditEntries, isConnected, address } = useBastionEVM();
  const [stats, setStats] = useState<StatsData | null>(null);
  const [paused, setPaused] = useState<boolean | null>(null);
  const [entries, setEntries] = useState<AuditEntryData[]>([]);
  const [loading, setLoading] = useState(false);

  const deployed = Boolean(AUDIT_ADDRESS);

  const load = useCallback(async () => {
    if (!deployed || !isConnected) return;
    setLoading(true);
    try {
      const [s, p, e] = await Promise.all([fetchStats(), fetchPaused(), fetchAuditEntries(25)]);
      setStats(s);
      setPaused(p);
      setEntries(e);
    } finally {
      setLoading(false);
    }
  }, [deployed, isConnected, fetchStats, fetchPaused, fetchAuditEntries]);

  useEffect(() => {
    load();
    const iv = setInterval(load, 30000);
    return () => clearInterval(iv);
  }, [load]);

  // Not deployed yet — honest placeholder, no crash.
  if (!deployed) {
    return (
      <div className="max-w-7xl mx-auto mb-4 rounded-xl p-6" style={CARD}>
        <p className="font-sans text-[10px] uppercase tracking-wider text-zinc-500 mb-2">EVM Networks (Sepolia / Base / Celo)</p>
        <p className="font-sans text-sm text-zinc-400">Bastion contracts are not yet deployed on this frontend.</p>
        <p className="font-sans text-xs text-zinc-600 mt-1">
          Set <code className="text-zinc-500">VITE_BASTION_AUDIT_ADDRESS</code> (and the policy / firewall / registry
          addresses) after the Sepolia deploy — see <code className="text-zinc-500">docs/EVM_READINESS.md §7</code>.
          Testnet-only; mainnet is behind the external-audit gate.
        </p>
      </div>
    );
  }

  return (
    <div className="max-w-7xl mx-auto mb-4 rounded-xl p-6" style={CARD}>
      <div className="flex items-center justify-between mb-4">
        <p className="font-sans text-[10px] uppercase tracking-wider text-zinc-500">EVM Networks (Sepolia / Base / Celo)</p>
        <div className="flex items-center gap-2">
          <span className="px-2 py-0.5 rounded-full text-[10px] font-sans font-semibold border"
            style={paused
              ? { background: 'rgba(239,68,68,0.1)', color: '#ef4444', borderColor: 'rgba(239,68,68,0.2)' }
              : { background: 'rgba(34,197,94,0.1)', color: '#22c55e', borderColor: 'rgba(34,197,94,0.2)' }}>
            {paused === null ? '...' : paused ? 'PAUSED' : 'LIVE'}
          </span>
          <a href={`${EXPLORER_URL}/address/${AUDIT_ADDRESS}`} target="_blank" rel="noreferrer"
            className="font-mono text-[9px] text-zinc-600 hover:text-zinc-400 no-underline">
            {AUDIT_ADDRESS?.slice(0, 8)}...↗
          </a>
        </div>
      </div>

      {!isConnected ? (
        <p className="font-sans text-sm text-zinc-400">Connect an EVM wallet to load live Sepolia state.</p>
      ) : (
        <>
          <div className="grid grid-cols-3 gap-3 mb-4">
            <div className="rounded-lg p-3" style={CARD}>
              <p className="font-sans text-[10px] uppercase tracking-wider text-zinc-500 mb-1">Audit Entries</p>
              <p className="font-mono text-xl font-bold tabular-nums" style={{ color: '#627EEA' }}>{stats?.total ?? '—'}</p>
            </div>
            <div className="rounded-lg p-3" style={CARD}>
              <p className="font-sans text-[10px] uppercase tracking-wider text-zinc-500 mb-1">Allowed</p>
              <p className="font-mono text-xl font-bold tabular-nums" style={{ color: '#22c55e' }}>{stats?.allowed ?? '—'}</p>
            </div>
            <div className="rounded-lg p-3" style={CARD}>
              <p className="font-sans text-[10px] uppercase tracking-wider text-zinc-500 mb-1">Blocked</p>
              <p className="font-mono text-xl font-bold tabular-nums" style={{ color: '#ef4444' }}>{stats?.blocked ?? '—'}</p>
            </div>
          </div>

          <p className="font-sans text-[10px] uppercase tracking-wider text-zinc-500 mb-2">
            Recent AuditRecorded Events {loading && <span className="text-zinc-600">(loading...)</span>}
          </p>
          <div className="rounded-lg overflow-hidden" style={{ border: '1px solid rgba(255,255,255,0.04)' }}>
            <table className="w-full text-left font-mono text-xs">
              <thead>
                <tr className="border-b border-white/[0.04] text-zinc-500">
                  <th className="py-2 px-4 font-normal">Time</th>
                  <th className="py-2 px-4 font-normal">Decision</th>
                  <th className="py-2 px-4 font-normal">Agent</th>
                  <th className="py-2 px-4 font-normal">Reason</th>
                </tr>
              </thead>
              <tbody>
                {entries.map((e) => (
                  <tr key={e.id} className="border-b border-white/[0.02] hover:bg-white/[0.02]">
                    <td className="py-1.5 px-4 text-zinc-500">{new Date(e.timestamp * 1000).toLocaleTimeString()}</td>
                    <td className="py-1.5 px-4"><span style={{ color: DECISION_COLORS[e.decision] || '#71717a' }}>{e.decision}</span></td>
                    <td className="py-1.5 px-4 text-zinc-500">{e.account?.slice(0, 10)}...</td>
                    <td className="py-1.5 px-4 text-zinc-400">{e.reason?.slice(0, 40)}</td>
                  </tr>
                ))}
                {entries.length === 0 && (
                  <tr><td colSpan={4} className="py-8 text-center text-zinc-600">No audit entries on Sepolia yet.</td></tr>
                )}
              </tbody>
            </table>
          </div>
          {address && <p className="font-mono text-[9px] text-zinc-600 mt-2">Connected: {address}</p>}
        </>
      )}
    </div>
  );
}
