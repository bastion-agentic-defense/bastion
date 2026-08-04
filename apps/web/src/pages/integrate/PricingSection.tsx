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

      <div
        className="rounded-xl p-4 mb-4"
        style={{ background: 'rgba(34, 197, 94, 0.06)', border: '1px solid rgba(34, 197, 94, 0.15)' }}
      >
        <p className="font-sans text-sm leading-relaxed" style={{ color: 'var(--text-primary)' }}>
          Bastion is a <strong>non-profit trust runtime</strong>, like Grafana.
          The core infrastructure is free and open source under Apache 2.0.
          Backend API calls are optionally paid via USDT or USDC to cover infrastructure costs.
          No tokens, no treasuries, no paywalls.
        </p>
      </div>

      <div className="overflow-x-auto">
        <table className="w-full text-left font-mono text-xs border-collapse">
          <thead>
            <tr style={{ borderBottom: '1px solid var(--border)' }}>
              <th className="py-2 pr-4 font-normal" style={{ color: 'var(--text-muted)' }}>Tool</th>
              <th className="py-2 pr-4 font-normal" style={{ color: 'var(--text-muted)' }}>Free per Month</th>
              <th className="py-2 font-normal" style={{ color: 'var(--text-muted)' }}>Price (USD)</th>
            </tr>
          </thead>
          <tbody>
            {[
              { tool: 'bastion_simulate_transaction', free: '100', usd: '$0.10', cat: 'paid' },
              { tool: 'bastion_override_block', free: '10', usd: '$1.00', cat: 'paid' },
              { tool: 'bastion_update_policy', free: '5', usd: '$5.00', cat: 'paid' },
              { tool: 'bastion_circuit_breaker_toggle', free: '3', usd: '$10.00', cat: 'paid' },
              { tool: 'bastion_get_policy', free: 'Unlimited', usd: 'Free', cat: 'free' },
              { tool: 'bastion_get_audit_logs', free: 'Unlimited', usd: 'Free', cat: 'free' },
              { tool: 'bastion_get_audit_stats', free: 'Unlimited', usd: 'Free', cat: 'free' },
              { tool: 'bastion_resolve_did', free: 'Unlimited', usd: 'Free', cat: 'free' },
            ].map((row) => (
              <tr key={row.tool} style={{ borderBottom: '1px solid var(--border)' }}>
                <td className="py-2 pr-4" style={{ color: row.cat === 'paid' ? 'var(--text-primary)' : 'var(--text-muted)' }}>
                  {row.tool}
                </td>
                <td className="py-2 pr-4" style={{ color: 'var(--text-primary)' }}>{row.free}</td>
                <td className="py-2" style={{ color: row.cat === 'paid' ? '#3b82f6' : '#22c55e' }}>{row.usd}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </section>
  );
}
