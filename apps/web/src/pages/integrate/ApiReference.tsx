import { useState } from 'react';

interface ApiMethod {
  method: string;
  signature: string;
  description: string;
  example: string;
}

const METHODS: ApiMethod[] = [
  {
    method: 'getEntryCount',
    signature: 'getEntryCount(): Promise<bigint>',
    description: 'Total number of audit entries recorded on-chain by the BastionAudit contract.',
    example: `import { BastionEVMClient } from "@zkos-labs/bastion-sdk";

const client = new BastionEVMClient({ publicClient, chain, contracts });
const total = await client.getEntryCount();
console.log(total);`,
  },
  {
    method: 'readAuditEntry',
    signature: 'readAuditEntry(entryId: Hex): Promise<BastionAuditEntry>',
    description: 'Read a single EIP-712 signed audit entry by its bytes32 id.',
    example: `const entry = await client.readAuditEntry(entryId);
console.log(entry.agent, entry.target, entry.allowed);`,
  },
  {
    method: 'readPolicy',
    signature: 'readPolicy(agent: Address): Promise<BastionPolicy>',
    description: 'Read the per-agent policy (limits, allowed targets/selectors, cooldown) from BastionPolicy.',
    example: `const policy = await client.readPolicy(agentAddress);
console.log(policy.maxValuePerTx, policy.allowedTargets);`,
  },
  {
    method: 'writePolicy',
    signature: 'writePolicy(agent: Address, policy: BastionPolicy): Promise<Hex>',
    description: 'Set the policy for an agent. Requires a wallet client (write).',
    example: `const hash = await client.writePolicy(agentAddress, {
  agent: agentAddress,
  isActive: true,
  maxValuePerTx: 1_000_000n,
  maxGasPerTx: 0n,
  dailyTxLimit: 100n,
  cooldownSeconds: 0n,
  allowedTargets: [targetAddress],
  allowedSelectors: [],
  extraData: "0x",
});`,
  },
  {
    method: 'validate',
    signature: 'validate(agent, target, value, callData): Promise<{ allowed: boolean; reason: Hex }>',
    description: 'Evaluate a transaction against the per-agent policy (view call to checkTransaction).',
    example: `const { allowed, reason } = await client.validate(
  agentAddress,
  targetAddress,
  100000n,
  callData,
);`,
  },
  {
    method: 'isPaused',
    signature: 'isPaused(): Promise<boolean>',
    description: 'Whether the ERC-7579 firewall validator is currently paused.',
    example: `const paused = await client.isPaused();`,
  },
  {
    method: 'pause',
    signature: 'pause(): Promise<Hex>',
    description: 'Pause the firewall validator. Circuit breaker for emergencies. Requires a wallet client.',
    example: `const hash = await client.pause();`,
  },
  {
    method: 'unpause',
    signature: 'unpause(): Promise<Hex>',
    description: 'Resume the firewall validator after a pause. Requires a wallet client.',
    example: `const hash = await client.unpause();`,
  },
  {
    method: 'simulateEvm',
    signature: 'simulateEvm(req: EvmSimulateRequest): Promise<EvmSimulateResponse>',
    description: 'Run an EVM transaction through the sidecar policy engine before signing.',
    example: `import { BastionSidecar } from "@zkos-labs/bastion-sdk";

const sidecar = new BastionSidecar({ baseUrl: SIDECAR_URL });
const result = await sidecar.simulateEvm({
  transaction,
  intent: "swap 1 ETH for USDC",
  chain: "ethereum",
});
console.log(result.decision, result.risk_score);`,
  },
  {
    method: 'getPolicy',
    signature: 'getPolicy(): Promise<SidecarPolicy>',
    description: 'Fetch the current sidecar policy (rate limits, allowlists, caps).',
    example: `const policy = await sidecar.getPolicy();`,
  },
  {
    method: 'updatePolicy',
    signature: 'updatePolicy(policy: Partial<SidecarPolicy>): Promise<SidecarPolicy>',
    description: 'Update the sidecar policy (max native per tx, rate limit, allowed programs).',
    example: `await sidecar.updatePolicy({
  max_sol_per_tx: 1,
  rate_limit_per_minute: 120,
  allowed_programs: ["0x..."],
});`,
  },
  {
    method: 'engageCircuitBreaker',
    signature: 'engageCircuitBreaker(): Promise<CircuitBreakerStatus>',
    description: 'Pause the protocol. Fail-closed breaker for emergencies.',
    example: `await sidecar.engageCircuitBreaker();`,
  },
  {
    method: 'disengageCircuitBreaker',
    signature: 'disengageCircuitBreaker(): Promise<CircuitBreakerStatus>',
    description: 'Resume the protocol after a pause.',
    example: `await sidecar.disengageCircuitBreaker();`,
  },
  {
    method: 'approve',
    signature: 'approve(req: OverrideRequest): Promise<SimulateResponse | { error: string }>',
    description: 'Human-in-the-loop override: ALLOW or REJECT a blocked transaction.',
    example: `await sidecar.approve({ block_id, action: "ALLOW" });`,
  },
  {
    method: 'verifyVerdict',
    signature: 'verifyVerdict(verdict: Verdict, attestation: VerdictAttestation): Promise<boolean>',
    description: 'ERC-8354: verify a confidential verdict signature against the agent key.',
    example: `import { verifyVerdict } from "@zkos-labs/bastion-sdk";

const ok = await verifyVerdict(verdict, attestation);`,
  },
  {
    method: 'computeCapabilityCommitment',
    signature: 'computeCapabilityCommitment(inputs: CapabilityInputs): Hex',
    description: 'ERC-8380: derive the unclonable capability commitment for an agent credential.',
    example: `import { computeCapabilityCommitment } from "@zkos-labs/bastion-sdk";

const commitment = computeCapabilityCommitment({
  agentId,
  homeChainId,
  homeDomainId,
  capabilityIndex,
  actionCommitment,
  executor,
});`,
  },
];

function ApiMethodCard({ method }: { method: ApiMethod }) {
  const [expanded, setExpanded] = useState(false);
  const [copied, setCopied] = useState(false);

  function handleCopy(e: React.MouseEvent) {
    e.stopPropagation();
    navigator.clipboard.writeText(method.example);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  }

  return (
    <div
      className="rounded-xl overflow-hidden transition-colors duration-150"
      style={{ background: 'var(--card-bg)', border: '1px solid var(--card-border)' }}
    >
      <button
        onClick={() => setExpanded(!expanded)}
        className="w-full text-left p-4 flex items-start justify-between gap-4 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-inset focus-visible:ring-[var(--accent)]"
        aria-expanded={expanded}
      >
        <div className="min-w-0">
          <code className="font-mono text-sm font-semibold" style={{ color: 'var(--text-primary)' }}>
            {method.method}()
          </code>
          <p className="font-sans text-xs mt-1" style={{ color: 'var(--text-muted)' }}>
            {method.description}
          </p>
        </div>
        <span
          className="font-mono text-xs flex-shrink-0 mt-0.5 transition-transform duration-150"
          style={{ color: 'var(--text-muted)', transform: expanded ? 'rotate(180deg)' : 'rotate(0deg)' }}
        >
          ▼
        </span>
      </button>

      {expanded && (
        <div style={{ borderTop: '1px solid var(--border)' }}>
          <div className="px-4 py-2" style={{ background: 'var(--bg-subtle)' }}>
            <code className="font-mono text-xs" style={{ color: 'var(--text-muted)' }}>
              {method.signature}
            </code>
          </div>
          <div className="relative">
            <button
              onClick={handleCopy}
              className="absolute top-2 right-2 font-sans text-xs font-medium px-2 py-0.5 rounded transition-colors duration-150 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-[var(--accent)] z-10"
              style={{ color: copied ? '#22c55e' : 'var(--text-muted)', background: 'var(--bg)' }}
            >
              {copied ? 'Copied' : 'Copy'}
            </button>
            <pre className="p-4 overflow-x-auto">
              <code className="font-mono text-sm leading-relaxed block" style={{ color: 'var(--text-primary)' }}>
                {method.example}
              </code>
            </pre>
          </div>
        </div>
      )}
    </div>
  );
}

export default function ApiReference() {
  return (
    <section className="max-w-3xl mx-auto" aria-labelledby="api-heading">
      <h3
        id="api-heading"
        className="font-sans text-sm uppercase tracking-wider mb-4"
        style={{ color: 'var(--text-muted)' }}
      >
        API Reference
      </h3>

      {/* MCP HTTP Endpoints */}
      <div className="mb-8">
        <p className="font-sans text-xs font-medium mb-3" style={{ color: 'var(--accent)' }}>MCP HTTP Server (proxied via sidecar)</p>
        <div className="space-y-1">
          {[
            { method: 'GET', path: '/mcp/health', desc: 'Health check' },
            { method: 'GET', path: '/mcp/sse', desc: 'SSE connection' },
            { method: 'POST', path: '/mcp/messages', desc: 'MCP JSON-RPC messages' },
            { method: 'GET', path: '/mcp/pricing', desc: 'Tool pricing + free tier' },
          ].map((e) => (
            <div key={e.path} className="flex items-center gap-3 font-mono text-xs py-1.5 px-3 rounded" style={{ background: 'var(--bg-subtle)' }}>
              <span className="w-10 shrink-0" style={{ color: e.method === 'GET' ? '#22c55e' : '#f59e0b' }}>{e.method}</span>
              <code className="text-zinc-300">{e.path}</code>
              <span className="ml-auto text-zinc-600">{e.desc}</span>
            </div>
          ))}
        </div>
      </div>

      {/* Sidecar REST Endpoints */}
      <div className="mb-8">
        <p className="font-sans text-xs font-medium mb-3" style={{ color: 'var(--accent)' }}>Sidecar REST API (:3000)</p>
        <div className="space-y-1">
          {[
            { method: 'POST', path: '/did/generate', desc: 'Generate Ed25519 keypair + DID' },
            { method: 'POST', path: '/auth/nonce', desc: 'Get challenge nonce for DID auth' },
            { method: 'POST', path: '/auth/verify', desc: 'Verify DID signature' },
            { method: 'POST', path: '/override', desc: 'HITL override' },
            { method: 'GET', path: '/logs', desc: 'Paginated audit logs' },
            { method: 'POST', path: '/agents', desc: 'Register agent (auth)' },
            { method: 'GET', path: '/agents', desc: 'List all agents' },
            { method: 'GET', path: '/agents/:did', desc: 'Agent detail' },
            { method: 'GET', path: '/agents/:did/audit', desc: 'Agent audit trail' },
            { method: 'GET', path: '/agents/:did/children', desc: 'List sub-agents' },
            { method: 'GET', path: '/agents/:did/tree', desc: 'Delegation tree' },
            { method: 'POST', path: '/agents/:did/delegate', desc: 'Spawn sub-agent (auth)' },
            { method: 'GET', path: '/agents/:did/stake', desc: 'Agent stake status' },
            { method: 'GET', path: '/policy', desc: 'Current policy' },
            { method: 'POST', path: '/policy/full', desc: 'Update policy (auth)' },
            { method: 'GET', path: '/circuit-breaker/status', desc: 'Breaker status' },
            { method: 'POST', path: '/circuit-breaker/engage', desc: 'Pause protocol (auth)' },
            { method: 'POST', path: '/circuit-breaker/disengage', desc: 'Resume protocol (auth)' },
            { method: 'GET', path: '/pending', desc: 'Pending approvals' },
            { method: 'GET', path: '/health', desc: 'Server health' },
            { method: 'GET', path: '/did/resolve/:did', desc: 'DID resolution' },
            { method: 'POST', path: '/api/v2/evaluate', desc: 'Evaluate transaction (v2)' },
            { method: 'POST', path: '/api/v2/simulate-evm', desc: 'Simulate EVM tx' },
            { method: 'GET', path: '/events', desc: 'SSE event stream' },
          ].map((e) => (
            <div key={e.path + e.method} className="flex items-center gap-3 font-mono text-xs py-1.5 px-3 rounded" style={{ background: 'var(--bg-subtle)' }}>
              <span className="w-10 shrink-0" style={{ color: e.method === 'GET' ? '#22c55e' : '#f59e0b' }}>{e.method}</span>
              <code className="text-zinc-300">{e.path}</code>
              <span className="ml-auto text-zinc-600">{e.desc}</span>
            </div>
          ))}
        </div>
      </div>

      {/* SDK Methods */}
      <p className="font-sans text-xs font-medium mb-3" style={{ color: 'var(--accent)' }}>TypeScript SDK (@zkos-labs/bastion-sdk)</p>

      <div className="space-y-2">
        {METHODS.map((m) => (
          <ApiMethodCard key={m.method} method={m} />
        ))}
      </div>
    </section>
  );
}
