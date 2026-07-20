export default function PricingSection() {
  return (
    <section className="max-w-3xl mx-auto" aria-labelledby="pricing-heading">
      <h3
        id="pricing-heading"
        className="font-sans text-sm uppercase tracking-wider mb-4"
        style={{ color: 'var(--text-muted)' }}
      >
        Tool Pricing
      </h3>

      <p className="font-sans text-sm mb-6 leading-relaxed" style={{ color: 'var(--text-muted)' }}>
        All read-only tools are free. Paid tools have a free monthly tier. Once exhausted, pay per call via an
        on-chain transfer to the treasury — <span style={{ color: 'var(--text-primary)' }}>Ethereum (ETH)</span> or
        Solana (SOL). USD is the anchor price; the ETH/SOL amounts are the equivalent you transfer.
      </p>

      {/* Pricing table */}
      <div className="overflow-x-auto mb-6">
        <table className="w-full text-left font-mono text-xs border-collapse">
          <thead>
            <tr style={{ borderBottom: '1px solid var(--border)' }}>
              <th className="py-2 pr-4 font-normal" style={{ color: 'var(--text-muted)' }}>Tool</th>
              <th className="py-2 pr-4 font-normal" style={{ color: 'var(--text-muted)' }}>Free/Month</th>
              <th className="py-2 pr-4 font-normal" style={{ color: 'var(--text-muted)' }}>Price (ETH)</th>
              <th className="py-2 pr-4 font-normal" style={{ color: 'var(--text-muted)' }}>Price (SOL)</th>
              <th className="py-2 font-normal" style={{ color: 'var(--text-muted)' }}>Price (USD)</th>
            </tr>
          </thead>
          <tbody>
            {[
              { tool: 'bastion_simulate_transaction', free: '100', eth: '0.00003', sol: '0.001', usd: '$0.10', cat: 'paid' },
              { tool: 'bastion_override_block', free: '10', eth: '0.0003', sol: '0.01', usd: '$1.00', cat: 'paid' },
              { tool: 'bastion_update_policy', free: '5', eth: '0.0015', sol: '0.05', usd: '$5.00', cat: 'paid' },
              { tool: 'bastion_circuit_breaker_toggle', free: '3', eth: '0.003', sol: '0.1', usd: '$10.00', cat: 'paid' },
              { tool: 'bastion_get_policy', free: '∞', eth: 'Free', sol: 'Free', usd: 'Free', cat: 'free' },
              { tool: 'bastion_get_audit_logs', free: '∞', eth: 'Free', sol: 'Free', usd: 'Free', cat: 'free' },
              { tool: 'bastion_get_audit_stats', free: '∞', eth: 'Free', sol: 'Free', usd: 'Free', cat: 'free' },
              { tool: 'bastion_resolve_did', free: '∞', eth: 'Free', sol: 'Free', usd: 'Free', cat: 'free' },
            ].map((row) => (
              <tr key={row.tool} style={{ borderBottom: '1px solid var(--border)' }}>
                <td className="py-2 pr-4" style={{ color: row.cat === 'paid' ? 'var(--text-primary)' : 'var(--text-muted)' }}>
                  {row.tool}
                </td>
                <td className="py-2 pr-4" style={{ color: 'var(--text-primary)' }}>{row.free}</td>
                <td className="py-2 pr-4" style={{ color: row.cat === 'paid' ? '#627EEA' : '#22c55e' }}>{row.eth}</td>
                <td className="py-2 pr-4" style={{ color: row.cat === 'paid' ? '#f59e0b' : '#22c55e' }}>{row.sol}</td>
                <td className="py-2" style={{ color: 'var(--text-muted)' }}>{row.usd}</td>
              </tr>
            ))}
          </tbody>
        </table>
        <p className="font-sans text-[10px] mt-2" style={{ color: 'var(--text-muted)' }}>
          ETH amounts assume ~$3,333/ETH and SOL ~$100/SOL; the server quotes the exact amount at request time.
        </p>
      </div>

      {/* x402 payment flow */}
      <div
        className="rounded-xl p-5 mb-4"
        style={{ background: 'var(--card-bg)', border: '1px solid var(--card-border)' }}
      >
        <h4 className="font-sans text-sm font-medium mb-3" style={{ color: 'var(--text-primary)' }}>
          x402 Payment Flow
        </h4>
        <div className="space-y-2 font-mono text-[10px]" style={{ color: 'var(--text-muted)' }}>
          <div>1. Transfer the quoted amount to the treasury for your chain:</div>
          <div className="pl-3">• <span style={{ color: '#627EEA' }}>Ethereum</span> (ETH): <code className="text-zinc-300">0x1B4d…TREASURY</code> <span className="text-zinc-600">(set via BASTION_ETH_TREASURY)</span></div>
          <div className="pl-3">• <span style={{ color: '#f59e0b' }}>Solana</span> (SOL): <code className="text-zinc-300">E9PsSz9XWgNR3TmSC57NHC2ZxJzF5NmbrWsDKEe7A7yM</code></div>
          <div>2. Retry with headers: <code className="text-green-400">X-Payment: &lt;tx_hash&gt;</code>, <code className="text-green-400">X-Payment-Chain: ethereum</code> <span className="text-zinc-600">(or <code className="text-green-400">solana</code>)</span></div>
          <div>3. Server verifies the on-chain transfer, executes tool</div>
        </div>
        <div className="mt-3 p-3 rounded font-mono text-[10px]" style={{ background: 'var(--bg-subtle)' }}>
          <span style={{ color: 'var(--text-muted)' }}>curl -X POST https://bastion-agentique.fly.dev/mcp/messages \</span><br />
          <span style={{ color: 'var(--text-muted)' }}>&nbsp;&nbsp;-H "X-Payment: 0x9f3a..." \</span><br />
          <span style={{ color: 'var(--text-muted)' }}>&nbsp;&nbsp;-H "X-Payment-Chain: ethereum" \</span><br />
          <span style={{ color: 'var(--text-muted)' }}>&nbsp;&nbsp;-d '&#123;&#123;"jsonrpc":"2.0",...&#125;&#125;'</span>
        </div>
      </div>

      {/* pay.sh */}
      <div
        className="rounded-xl p-5"
        style={{ background: 'var(--card-bg)', border: '1px solid var(--card-border)' }}
      >
        <h4 className="font-sans text-sm font-medium mb-3" style={{ color: 'var(--text-primary)' }}>
          Pay with pay.sh
        </h4>
        <p className="font-sans text-xs mb-3" style={{ color: 'var(--text-muted)' }}>
          Install pay.sh and call Bastion with automatic payment handling — no manual ETH/SOL transfer needed.
        </p>
        <pre className="p-3 rounded font-mono text-[10px] overflow-x-auto" style={{ background: 'var(--bg-subtle)', color: 'var(--text-primary)' }}>
{`# Start the Bastion pay.sh gateway
pay --sandbox server start bastion-provider.yml

# Call a paid endpoint (payment handled automatically)
pay --sandbox curl -X POST http://127.0.0.1:1402/v1/simulate \\
  -d '{"transaction":"...","intent":"swap 0.1 ETH for USDC"}'

# Call a free endpoint
pay --sandbox curl http://127.0.0.1:1402/v1/health`}
        </pre>
        <p className="font-sans text-[10px] mt-3" style={{ color: 'var(--text-muted)' }}>
          <a href="https://pay.sh" target="_blank" rel="noopener noreferrer" className="underline" style={{ color: 'var(--accent)' }}>pay.sh docs</a> — Provider spec at <code className="text-zinc-400">packages/mcp-server/bastion-provider.yml</code>
        </p>
      </div>
    </section>
  );
}
