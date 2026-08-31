import { useState } from 'react';
import { useSidecar } from '../../hooks/useSidecar';

export default function LiveTest() {
  const [status, setStatus] = useState<'idle' | 'loading' | 'ok' | 'error'>('idle');
  const [message, setMessage] = useState('');
  const [simTx, setSimTx] = useState('');
  const [simIntent, setSimIntent] = useState('');
  const [simChain, setSimChain] = useState('base');
  const [simStatus, setSimStatus] = useState<'idle' | 'loading' | 'ok' | 'error'>('idle');
  const [simResult, setSimResult] = useState<{
    passed: boolean;
    decision: string;
    reasoning: string;
    logs: string[];
  } | null>(null);

  const sidecar = useSidecar();

  async function handleTest() {
    setStatus('loading');
    setMessage('');
    try {
      const ok = await sidecar.fetchHealth();
      if (ok) {
        setStatus('ok');
        setMessage('Bastion sidecar is reachable and healthy.');
      } else {
        setStatus('error');
        setMessage('Sidecar health check failed (non-2xx response).');
      }
    } catch (err) {
      setStatus('error');
      setMessage('Could not reach the Bastion sidecar. Check your network.');
      console.error('[Bastion] LiveTest error:', err);
    }
  }

  const isSolana = simChain.trim().toLowerCase() === 'solana';

  async function handleSimulate() {
    setSimStatus('loading');
    setSimResult(null);
    try {
      let parsed: Record<string, unknown>;
      try {
        parsed = JSON.parse(simTx.trim());
      } catch {
        setSimStatus('error');
        setSimResult({
          passed: false,
          decision: 'ERROR',
          reasoning: isSolana
            ? 'Invalid JSON. Expected { "to": "<base58 pubkey>", "amount": 1000000 }'
            : 'Invalid JSON. Expected { "from": "0x..", "to": "0x..", "value": "0x0", "data": "0x" }',
          logs: [],
        });
        return;
      }

      let result;
      if (isSolana) {
        const to = parsed.to;
        if (typeof to !== 'string' || !to) {
          setSimStatus('error');
          setSimResult({
            passed: false,
            decision: 'ERROR',
            reasoning: 'Missing required field "to" (base58 Solana address).',
            logs: [],
          });
          return;
        }
        result = await sidecar.simulateSolana(
          {
            to,
            amount: typeof parsed.amount === 'number' ? parsed.amount : undefined,
            transaction: typeof parsed.transaction === 'string' ? parsed.transaction : undefined,
          },
          simIntent.trim() || undefined,
        );
      } else {
        const tx = parsed as { from?: string; to?: string; value?: string; data?: string };
        if (!tx.from || !tx.to) {
          setSimStatus('error');
          setSimResult({
            passed: false,
            decision: 'ERROR',
            reasoning: 'Missing required fields "from" and "to".',
            logs: [],
          });
          return;
        }
        result = await sidecar.simulateEvm(
          tx as { from: string; to: string; value?: string; data?: string },
          simIntent.trim() || undefined,
          simChain.trim() || 'base',
        );
      }

      if (!result) {
        setSimStatus('error');
        setSimResult({
          passed: false,
          decision: 'ERROR',
          reasoning: 'Could not reach the sidecar. Ensure it is running on port 3000.',
          logs: [],
        });
        return;
      }
      setSimStatus(result.allowed ? 'ok' : 'error');
      setSimResult({
        passed: result.allowed,
        decision: result.decision,
        reasoning: result.reason ?? (result.allowed ? 'Transaction passed all checks' : 'Transaction blocked'),
        logs: result.logs ?? [],
      });
    } catch (err) {
      setSimStatus('error');
      setSimResult({
        passed: false,
        decision: 'ERROR',
        reasoning: `Network error: ${String(err)}`,
        logs: [],
      });
    }
  }

  const decisionColor = simResult?.passed ? '#22c55e' : '#ef4444';

  return (
    <section className="max-w-3xl mx-auto space-y-12" aria-labelledby="test-heading">
      <h3
        id="test-heading"
        className="font-sans text-sm uppercase tracking-wider mb-4"
        style={{ color: 'var(--text-muted)' }}
      >
        Live Test
      </h3>

      {/* RPC Health Check */}
      <div
        className="rounded-xl p-6"
        style={{ background: 'var(--card-bg)', border: '1px solid var(--card-border)' }}
      >
        <h4 className="font-sans text-base font-medium mb-2" style={{ color: 'var(--text-primary)' }}>
          Bastion Sidecar Connection
        </h4>
        <p className="font-sans text-sm mb-4" style={{ color: 'var(--text-muted)' }}>
          Verify your connection to the Bastion sidecar before integrating.
        </p>
        <button
          onClick={handleTest}
          disabled={status === 'loading'}
          className="rounded-full px-6 py-2.5 text-sm font-medium font-sans transition-all duration-150 hover:opacity-90 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-[var(--accent)] focus-visible:ring-offset-2 disabled:opacity-50"
          style={{ background: 'var(--text-primary)', color: 'var(--bg)' }}
        >
          {status === 'loading' ? 'Testing...' : 'Test Connection'}
        </button>
        {message && (
          <p className="font-sans text-sm mt-4" style={{ color: status === 'ok' ? '#22c55e' : '#ef4444' }}>
            {message}
          </p>
        )}
      </div>

      {/* Sidecar Simulation Test */}
      <div
        className="rounded-xl p-6"
        style={{ background: 'var(--card-bg)', border: '1px solid var(--card-border)' }}
      >
        <h4 className="font-sans text-base font-medium mb-2" style={{ color: 'var(--text-primary)' }}>
          Simulate Transaction
        </h4>
        <p className="font-sans text-sm mb-4" style={{ color: 'var(--text-muted)' }}>
          {isSolana
            ? 'Paste a Solana transaction request (JSON) and run it through the Bastion sidecar firewall.'
            : 'Paste an EVM transaction (JSON) and run it through the Bastion sidecar firewall.'}
        </p>

        <div className="space-y-3 mb-4">
          <textarea
            value={simTx}
            onChange={(e) => setSimTx(e.target.value)}
            placeholder={
              isSolana
                ? '{ "to": "<base58 pubkey>", "amount": 1000000 }'
                : '{ "from": "0x..", "to": "0x..", "value": "0x0", "data": "0x" }'
            }
            rows={3}
            className="w-full p-3 rounded-lg font-mono text-sm resize-y"
            style={{
              background: 'var(--bg-subtle)',
              border: '1px solid var(--border)',
              color: 'var(--text-primary)',
            }}
          />
          <input
            value={simChain}
            onChange={(e) => setSimChain(e.target.value)}
            placeholder="Chain (e.g. base, celo, ethereum, monad, polygon, arbitrum, solana)"
            className="w-full p-3 rounded-lg font-mono text-sm"
            style={{
              background: 'var(--bg-subtle)',
              border: '1px solid var(--border)',
              color: 'var(--text-primary)',
            }}
          />
          {isSolana && (
            <p className="font-sans text-xs" style={{ color: 'var(--text-muted)' }}>
              Solana mode: "to" must be a base58-encoded address, not a 0x-hex EVM address.
            </p>
          )}
          <input
            value={simIntent}
            onChange={(e) => setSimIntent(e.target.value)}
            placeholder="Intent (optional, e.g. 'swap 0.1 ETH for USDC')"
            className="w-full p-3 rounded-lg font-mono text-sm"
            style={{
              background: 'var(--bg-subtle)',
              border: '1px solid var(--border)',
              color: 'var(--text-primary)',
            }}
          />
        </div>

        <button
          onClick={handleSimulate}
          disabled={simStatus === 'loading' || !simTx.trim()}
          className="rounded-full px-6 py-2.5 text-sm font-medium font-sans transition-all duration-150 hover:opacity-90 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-[var(--accent)] focus-visible:ring-offset-2 disabled:opacity-50 mb-4"
          style={{ background: 'var(--text-primary)', color: 'var(--bg)' }}
        >
          {simStatus === 'loading' ? 'Simulating...' : 'Run Simulation'}
        </button>

        {simResult && (
          <div
            className="rounded-lg p-4 mt-4"
            style={{
              background: 'var(--bg-subtle)',
              border: `1px solid ${decisionColor}`,
            }}
          >
            <div className="flex items-center gap-2 mb-2">
              <span
                className="font-mono text-xs font-bold px-2 py-0.5 rounded"
                style={{ background: decisionColor, color: '#fff' }}
              >
                {simResult.decision}
              </span>
              <span className="font-sans text-sm" style={{ color: 'var(--text-primary)' }}>
                {simResult.reasoning}
              </span>
            </div>
            {simResult.logs.length > 0 && (
              <details className="mt-2">
                <summary className="font-sans text-xs cursor-pointer" style={{ color: 'var(--text-muted)' }}>
                  Simulation logs ({simResult.logs.length} lines)
                </summary>
                <div
                  className="mt-2 p-3 rounded font-mono text-xs max-h-48 overflow-y-auto"
                  style={{ background: 'var(--bg)', color: 'var(--text-muted)' }}
                >
                  {simResult.logs.map((line, i) => (
                    <div key={i}>{line}</div>
                  ))}
                </div>
              </details>
            )}
          </div>
        )}
      </div>
    </section>
  );
}
