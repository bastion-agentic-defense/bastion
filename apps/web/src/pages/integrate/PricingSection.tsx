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
          Bastion is an <strong>open-source community project</strong> developed by ZKOS Labs.
          Released under Apache 2.0. Free to use, modify, self-host, and redistribute.
          No protocol fees, tokens, ICOs, or mandatory hosted services.
        </p>
        <p className="font-sans text-xs mt-2 leading-relaxed" style={{ color: 'var(--text-muted)' }}>
          The hosted sidecar is provided as a convenience. A small number of compute-intensive
          operations have usage limits and optional pricing after a generous free tier.
          Developers who self-host Bastion retain full functionality without any platform fees.
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

      <div className="mt-6 text-center">
        <a
          href="https://github.com/sponsors/zkos-labs"
          target="_blank"
          rel="noopener noreferrer"
          className="inline-flex items-center gap-2 px-4 py-2 rounded-lg font-sans text-sm no-underline"
          style={{ background: 'rgba(245, 158, 11, 0.12)', color: '#f59e0b', border: '1px solid rgba(245, 158, 11, 0.25)' }}
        >
          ♥ Donate to support Bastion development
        </a>
      </div>
    </section>
  );
}
