import { Link } from 'react-router-dom';
import { BastionMark, BastionLockup } from '../components/BastionMark';
import Ferrofluid from '../components/Ferrofluid';

/* ── Content ─────────────────────────────────────────────────────────────────
 * Every claim is drawn from README.md and docs/. Status markers are the
 * repository's own. Nothing is promoted beyond what the code does today, and
 * no third-party company, customer or investor is named.
 * ─────────────────────────────────────────────────────────────────────────── */

const PROBLEMS = [
  {
    title: 'Unbounded authority',
    body: 'An agent holding a key can call any contract, sign any payload, and reach any endpoint. The blast radius of a bad inference is the whole wallet.',
  },
  {
    title: 'Failure you cannot see',
    body: 'When an autonomous system does the wrong thing, the loss is discovered downstream - after settlement, in a reconciliation, or by a customer.',
  },
  {
    title: 'No record worth trusting',
    body: 'Application logs are written by the same process that made the decision. They prove nothing to an auditor, a regulator, or a counterparty.',
  },
  {
    title: 'Policy bolted on last',
    body: 'Limits live scattered across prompts, wrappers and review queues, so they are inconsistent, unenforceable, and quietly bypassed under load.',
  },
];

const PIPELINE = ['INTENT', 'POLICY', 'SIMULATION', 'REVIEW', 'AUDIT'];

const CASES = [
  {
    title: 'Treasury and payment agents',
    body: 'Cap the value any single agent can move, restrict it to known programs and counterparties, and hold anything above a threshold for a human.',
    best: 'Best suited for: finance teams running autonomous payouts, rebalancing, or vendor settlement.',
  },
  {
    title: 'DeFi and market operations',
    body: 'Screen every intent for flash loans, slippage, and authority changes before signing, with simulation against live chain state on each call.',
    best: 'Best suited for: market makers, yield strategies, and protocol operations running unattended.',
  },
  {
    title: 'Agent fleets and delegation',
    body: 'Give each agent an identity, delegate a bounded subset of authority to children, and stop the entire fleet with one circuit breaker.',
    best: 'Best suited for: platforms operating many agents on behalf of many end users.',
  },
];

const FOUNDATIONS = [
  {
    title: 'Programmable policy',
    body: 'Caps, rate limits, program allowlists and egress rules, evaluated before a signature exists.',
  },
  {
    title: 'Verifiable audit',
    body: 'Every verdict written on-chain with its reasoning intact, readable by anyone without trusting the runtime.',
  },
  {
    title: 'Portable identity',
    body: 'W3C DID-compliant agent identity, with reputation that accrues across chains rather than per application.',
  },
  {
    title: 'Human authority',
    body: 'Review queues for what policy will not decide alone, and a fleet-wide stop that overrides everything.',
  },
];

const STANDARDS = [
  { id: 'ERC-4337', adds: 'Policy-aware execution and multi-chain routing' },
  { id: 'ERC-7579', adds: 'Trust modules and policy validators' },
  { id: 'ERC-8004', adds: 'Runtime authorization across standards' },
  { id: 'EAS', adds: 'Lifecycle orchestration and execution evidence' },
  { id: 'Lit Protocol', adds: 'Confidential execution policies' },
  { id: 'EigenLayer', adds: 'Coordination using cryptoeconomic trust' },
];

const STATUS = [
  { label: 'Programmable policy engine', state: 'shipped' },
  { label: 'Transaction simulation and verification', state: 'shipped' },
  { label: 'Human-in-the-loop approvals', state: 'shipped' },
  { label: 'Verifiable trust ledger', state: 'shipped' },
  { label: 'TypeScript SDK', state: 'shipped' },
  { label: 'Agent identity and delegation', state: 'partial' },
  { label: 'Multi-chain execution planning', state: 'partial' },
  { label: 'Web2 API policy gateway', state: 'partial' },
  { label: 'MCP server', state: 'partial' },
  { label: 'Durable workflow execution', state: 'planned' },
  { label: 'Confidential computation', state: 'planned' },
  { label: 'Secrets management', state: 'planned' },
];

const STATE_COPY: Record<string, string> = {
  shipped: 'Shipped',
  partial: 'Partial',
  planned: 'Planned',
};

const STATE_COLOR: Record<string, string> = {
  shipped: 'var(--verdict-allow)',
  partial: 'var(--verdict-review)',
  planned: 'var(--text-faint)',
};

/* ── In-house iconography ────────────────────────────────────────────────────
 * Drawn for this brand on one 32-unit grid with a single stroke weight and
 * round joins, sharing the bracket construction of the Bastion mark. Not an
 * icon-pack import, and never set inside a tile.
 * ─────────────────────────────────────────────────────────────────────────── */
function FoundationIcon({ index }: { index: number }) {
  const common = {
    fill: 'none',
    stroke: 'currentColor',
    strokeWidth: 1.5,
    strokeLinecap: 'round' as const,
    strokeLinejoin: 'round' as const,
  };
  return (
    <svg width="32" height="32" viewBox="0 0 32 32" aria-hidden="true" style={{ color: 'var(--accent-text)' }}>
      {index === 0 && (
        <>
          <path d="M12 5 L6 9.5 L6 22.5 L12 27" {...common} />
          <path d="M20 5 L26 9.5 L26 22.5 L20 27" {...common} />
          <path d="M16 12.5 L16 19.5" {...common} />
        </>
      )}
      {index === 1 && (
        <>
          <path d="M6 24 L6 8 L26 8 L26 24" {...common} />
          <path d="M6 24 L26 24" {...common} />
          <path d="M11 19 L15 15 L19 18 L23 12" {...common} />
        </>
      )}
      {index === 2 && (
        <>
          <path d="M16 5 L16 27" {...common} />
          <path d="M8 11 L16 15.5 L24 11" {...common} />
          <path d="M8 20 L16 24.5 L24 20" {...common} />
        </>
      )}
      {index === 3 && (
        <>
          <path d="M16 5 L26 9.5 L26 18 C26 23 21 26 16 27.5 C11 26 6 23 6 18 L6 9.5 Z" {...common} />
          <path d="M12 16 L15 19 L20 13.5" {...common} />
        </>
      )}
    </svg>
  );
}

/* ── Navigation ──────────────────────────────────────────────────────────── */

function Nav() {
  return (
    <nav
      className="fixed top-0 inset-x-0 z-50"
      aria-label="Main"
      style={{ background: 'var(--bg)', borderBottom: '1px solid var(--border)' }}
    >
      <div className="max-w-[1180px] mx-auto px-6 sm:px-10 h-[68px] flex items-center justify-between gap-6">
        <a href="#main" className="no-underline" style={{ color: 'var(--text-primary)' }} aria-label="Bastion, home">
          <BastionLockup size={23} markColor="var(--accent)" />
        </a>

        <div className="hidden md:flex items-center gap-9">
          {[
            ['Runtime', '#runtime'],
            ['Standards', '#standards'],
            ['Status', '#status'],
            ['Docs', '/docs'],
          ].map(([label, href]) => (
            <a
              key={label}
              href={href}
              className="font-sans no-underline nav-link"
              style={{ fontSize: '14px', color: 'var(--text-secondary)' }}
            >
              {label}
            </a>
          ))}
        </div>

        <Link
          to="/integrate"
          className="font-sans no-underline"
          style={{
            fontSize: '14px',
            fontWeight: 500,
            padding: '0.62rem 1.35rem',
            borderRadius: '999px',
            background: 'var(--accent)',
            color: 'oklch(1 0 0)',
            textDecoration: 'none',
          }}
        >
          Integrate
        </Link>
      </div>
    </nav>
  );
}

/* ── Page ────────────────────────────────────────────────────────────────── */

export default function Landing() {
  return (
    // Ink is the page's base ground; the warm and stone bands below read as lit
    // panels set into it.
    <div className="band-ink">
      <Nav />

      <main id="main">
        {/* ── Hero ── */}
        <section
          className="hero-gradient grain overflow-hidden"
          style={{ marginTop: '68px', minHeight: 'min(76vh, 640px)', display: 'flex', alignItems: 'center' }}
        >
          {/* Living background: a ferrofluid shimmer in white over the orange
              field. Absolutely positioned behind the content, decorative only -
              the headline and CTA never depend on it. */}
          <div
            aria-hidden="true"
            style={{ position: 'absolute', inset: 0, zIndex: 0, pointerEvents: 'none' }}
          >
            <Ferrofluid
              colors={['#ffffff', '#ffffff', '#ffffff']}
              speed={0.5}
              scale={1.6}
              turbulence={1}
              fluidity={0.1}
              rimWidth={0.2}
              sharpness={2.5}
              shimmer={1.5}
              glow={2}
              flowDirection="down"
              opacity={0.55}
              mouseInteraction={false}
            />
          </div>

          <div className="relative z-10 w-full max-w-[1180px] mx-auto px-6 sm:px-10 py-24">
            <div className="parallax-lead">
              <h1
                className="font-display"
                style={{
                  fontSize: 'clamp(2.2rem, 4.6vw, 4rem)',
                  lineHeight: 1.08,
                  letterSpacing: '-0.034em',
                  fontWeight: 500,
                  color: 'oklch(1 0 0)',
                  maxWidth: '17ch',
                  margin: 0,
                }}
              >
                Trusted execution infrastructure for autonomous agents
              </h1>

              <p
                className="font-sans mt-7"
                style={{ fontSize: '16.5px', lineHeight: 1.6, color: 'oklch(1 0 0 / 0.92)', maxWidth: '42ch' }}
              >
                Enforce policy, simulate every transaction, and keep a verifiable
                audit trail across your agent fleet.
              </p>

              <p
                className="font-sans mt-4 inline-flex items-center gap-2 px-3 py-1.5 rounded-full"
                style={{ fontSize: '11.5px', color: 'oklch(1 0 0 / 0.82)', background: 'oklch(1 0 0 / 0.10)', width: 'fit-content' }}
              >
                Open-source community project by ZKOS Labs. Self-host or use the hosted sidecar.
              </p>

              <div className="mt-10 flex flex-wrap items-center gap-7">
                <Link to="/integrate" className="btn-on-gradient">
                  Integrate your agent
                </Link>
                <a
                  href="https://github.com/zkos-labs/bastion"
                  target="_blank"
                  rel="noopener noreferrer"
                  className="font-sans no-underline"
                  style={{ fontSize: '14px', color: 'oklch(1 0 0 / 0.88)', textDecoration: 'none' }}
                >
                  Source ↗
                </a>
                <a
                  href="https://github.com/sponsors/zkos-labs"
                  target="_blank"
                  rel="noopener noreferrer"
                  className="font-sans no-underline"
                  style={{ fontSize: '14px', color: 'oklch(1 0 0 / 0.88)', textDecoration: 'none' }}
                >
                  Donate ♥
                </a>
              </div>
            </div>
          </div>
        </section>

        {/* ── Runs on ──
          * The reference runs an investor wall here. Bastion has none to show and
          * inventing one would be a lie, so the beat carries something true. */}
        <section className="py-12">
          <p className="band-kicker text-center">Runs on</p>
          <div className="marquee" aria-label="Runs on: Ethereum, Monad, Base, Celo, zkSync, Robinhood.">
            <div className="marquee-track marquee-track--reverse" aria-hidden="true">
              {Array.from({ length: 6 }).flatMap(() =>
                ['Ethereum', 'Monad', 'Base', 'Celo', 'zkSync', 'Robinhood'],
              ).map((chain, i) => (
                <span key={i} className="inline-flex items-center gap-10">
                  <span
                    className="font-display"
                    style={{ fontSize: '21px', fontWeight: 500, color: 'var(--text-muted)', letterSpacing: '-0.02em' }}
                  >
                    {chain}
                  </span>
                  <BastionMark size={13} color="var(--text-faint)" />
                </span>
              ))}
            </div>
          </div>
        </section>

        {/* ── Statement ── */}
        <section>
          <div className="max-w-[1000px] mx-auto px-6 sm:px-10 pb-24 sm:pb-28">
            <p
              className="font-display text-center"
              style={{
                fontSize: 'clamp(1.5rem, 3.1vw, 2.4rem)',
                lineHeight: 1.24,
                letterSpacing: '-0.028em',
                margin: 0,
              }}
            >
              The full-stack runtime to evaluate, simulate, approve, and prove every
              action an autonomous agent takes.
            </p>
          </div>
        </section>

        {/* ── The problem ── */}
        <section className="band-ink">
          <div className="max-w-[1180px] mx-auto px-6 sm:px-10 py-24 sm:py-32">
            <p className="band-kicker">The problem</p>
            <div
              className="grid gap-x-16 gap-y-14"
              style={{ gridTemplateColumns: 'repeat(auto-fit, minmax(min(100%, 380px), 1fr))' }}
            >
              <h2
                className="band-title font-display"
                style={{
                  fontSize: 'clamp(1.8rem, 3.4vw, 2.7rem)',
                  lineHeight: 1.14,
                  letterSpacing: '-0.03em',
                  maxWidth: '17ch',
                  margin: 0,
                }}
              >
                You already know agents will run production. Here is what stands in
                the way.
              </h2>

              <div
                className="grid gap-x-12 gap-y-11"
                style={{ gridTemplateColumns: 'repeat(auto-fit, minmax(min(100%, 230px), 1fr))' }}
              >
                {PROBLEMS.map(p => (
                  <div key={p.title}>
                    <h3
                      className="font-display"
                      style={{ fontSize: '1.2rem', letterSpacing: '-0.022em', margin: '0 0 0.7rem' }}
                    >
                      {p.title}
                    </h3>
                    <p className="band-body font-sans" style={{ fontSize: '14.5px', lineHeight: 1.65, margin: 0 }}>
                      {p.body}
                    </p>
                  </div>
                ))}
              </div>
            </div>
          </div>
        </section>

        {/* ── Pipeline marquee ──
          * The stages an intent passes through, running continuously. The text is
          * present and legible at every frame; only its offset animates. */}
        <section className="marquee py-10" style={{ borderBottom: '1px solid var(--border)' }} aria-hidden="true">
          <div className="marquee-track">
            {[...PIPELINE, ...PIPELINE, ...PIPELINE, ...PIPELINE].map((stage, i) => (
              <span key={i} className="inline-flex items-center gap-10">
                <span
                  className="font-display"
                  style={{
                    fontSize: 'clamp(2.4rem, 6vw, 5rem)',
                    fontWeight: 500,
                    letterSpacing: '-0.03em',
                    color: i % PIPELINE.length === 0 ? 'var(--accent)' : 'var(--text-primary)',
                  }}
                >
                  {stage}
                </span>
                <span style={{ fontSize: 'clamp(1.6rem, 3.4vw, 2.6rem)', color: 'var(--text-faint)' }}>→</span>
              </span>
            ))}
          </div>
        </section>

        {/* ── Core capabilities ── */}
        <section id="runtime" className="band-ink">
          <div className="max-w-[1180px] mx-auto px-6 sm:px-10 py-24 sm:py-32">
            <p className="band-kicker">Core capabilities</p>
            <h2
              className="font-display"
              style={{
                fontSize: 'clamp(1.9rem, 4vw, 3rem)',
                letterSpacing: '-0.032em',
                lineHeight: 1.1,
                margin: 0,
                color: 'var(--accent-text)',
              }}
            >
              Four controls. One runtime.
            </h2>
            <p
              className="font-sans mt-5"
              style={{ fontSize: '15.5px', lineHeight: 1.7, color: 'var(--text-secondary)', maxWidth: '52ch' }}
            >
              Adopt a single control or all of them. They compose into one decision
              path that sits between your agent and everything it can reach.
            </p>

            {/* Capability 1 - the ledger */}
            <div className="panel mt-16 p-8 sm:p-11">
              <div style={{ maxWidth: '62ch' }}>
                <p className="band-kicker" style={{ color: 'var(--accent-text)' }}>
                  Policy and simulation
                </p>
                <h3
                  className="font-display"
                  style={{ fontSize: 'clamp(1.5rem, 2.6vw, 2rem)', letterSpacing: '-0.028em', lineHeight: 1.16, margin: 0 }}
                >
                  <span style={{ color: 'var(--accent-text)' }}>Every decision</span> carries
                  the rule that made it.
                </h3>
                <ul style={{ listStyle: 'none', margin: '2rem 0 0', padding: 0 }}>
                  {[
                    'Native-token caps, rate limits, program allowlists and egress rules.',
                    'Simulation against live chain state before a signature exists.',
                    'Flash loan, slippage and authority-change screening on every intent.',
                  ].map(line => (
                    <li key={line} className="flex items-start gap-3" style={{ marginBottom: '0.85rem' }}>
                      <BastionMark size={15} color="var(--accent)" />
                      <span className="font-sans" style={{ fontSize: '14.5px', lineHeight: 1.6, color: 'var(--text-secondary)' }}>
                        {line}
                      </span>
                    </li>
                  ))}
                </ul>
              </div>

            </div>

            {/* Capability 2 - identity and delegation */}
            <div
              className="panel mt-8 p-8 sm:p-11 grid gap-x-14 gap-y-10 items-center"
              style={{ gridTemplateColumns: 'repeat(auto-fit, minmax(min(100%, 340px), 1fr))' }}
            >
              <div>
                <p className="band-kicker" style={{ color: 'var(--accent-text)' }}>
                  Identity and delegation
                </p>
                <h3
                  className="font-display"
                  style={{ fontSize: 'clamp(1.5rem, 2.6vw, 2rem)', letterSpacing: '-0.028em', lineHeight: 1.16, margin: 0 }}
                >
                  <span style={{ color: 'var(--accent-text)' }}>Bounded authority</span>, handed
                  down a tree.
                </h3>
                <ul style={{ listStyle: 'none', margin: '2rem 0 0', padding: 0 }}>
                  {[
                    'W3C DID-compliant identity issued per agent, portable across chains.',
                    'Children inherit a subset of a parent policy and can never exceed it.',
                    'Revocation takes effect on the next evaluation, not the next deploy.',
                  ].map(line => (
                    <li key={line} className="flex items-start gap-3" style={{ marginBottom: '0.85rem' }}>
                      <BastionMark size={15} color="var(--accent)" />
                      <span className="font-sans" style={{ fontSize: '14.5px', lineHeight: 1.6, color: 'var(--text-secondary)' }}>
                        {line}
                      </span>
                    </li>
                  ))}
                </ul>
              </div>

              <div className="report-frame">
                <div className="report-chrome">Example reporting</div>
                <div className="p-6">
                  <p className="band-kicker" style={{ margin: '0 0 0.4rem' }}>
                    Delegation tree
                  </p>
                  <p className="font-display" style={{ fontSize: '1.9rem', margin: '0 0 1.5rem' }}>
                    4 agents
                  </p>
                  {[
                    { did: 'treasury-01', scope: 'root · 5.0 SOL cap', depth: 0 },
                    { did: 'ops-payer-02', scope: 'delegated · 1.0 SOL cap', depth: 1 },
                    { did: 'mm-relay-04', scope: 'delegated · swap only', depth: 1 },
                    { did: 'agent-8f2c', scope: 'revoked', depth: 2 },
                  ].map(row => (
                    <div
                      key={row.did}
                      className="flex items-baseline justify-between gap-4 py-2.5"
                      style={{ borderTop: '1px solid var(--card-border)', paddingLeft: `${row.depth * 1.1}rem` }}
                    >
                      <span className="font-mono" style={{ fontSize: '12px', color: 'var(--text-primary)' }}>
                        {row.did}
                      </span>
                      <span className="font-sans" style={{ fontSize: '12.5px', color: 'var(--text-muted)' }}>
                        {row.scope}
                      </span>
                    </div>
                  ))}
                </div>
              </div>
            </div>
          </div>
        </section>

        {/* ── TrustIntent ──
          * Declarative intent over imperative API calls. The strategic abstraction
          * that separates what an agent wants from how Bastion executes it. */}
        <section className="band-stone">
          <div className="max-w-[1180px] mx-auto px-6 sm:px-10 py-24 sm:py-32">
            <p className="band-kicker">Abstraction</p>
            <h2
              className="font-display"
              style={{
                fontSize: 'clamp(1.8rem, 3.2vw, 2.5rem)',
                letterSpacing: '-0.03em',
                lineHeight: 1.14,
                margin: '0 0 1.6rem',
                maxWidth: '20ch',
              }}
            >
              Agents declare what they want. Bastion determines how to execute it safely.
            </h2>
            <p
              className="font-sans"
              style={{ fontSize: '15.5px', lineHeight: 1.65, color: 'var(--text-secondary)', maxWidth: '56ch', margin: '0 0 3.5rem' }}
            >
              Instead of agents writing low-level transaction calls and API
              requests, they submit a TrustIntent - a declarative specification
              of what should happen. Bastion resolves the chain, the wallet
              strategy, the policy checks, the simulation, and the human gate.
            </p>

            <div className="grid gap-x-16 gap-y-10" style={{ gridTemplateColumns: 'repeat(auto-fit, minmax(min(100%, 340px), 1fr))' }}>
              <div>
                <p className="font-sans font-semibold" style={{ fontSize: '13.5px', color: 'var(--text-muted)', margin: '0 0 0.6rem', letterSpacing: '0.04em' }}>
                  WHAT AGENTS WRITE TODAY
                </p>
                <pre
                  className="font-mono"
                  style={{
                    fontSize: '13px',
                    lineHeight: 1.6,
                    background: 'var(--panel)',
                    padding: '1.4rem',
                    borderRadius: '8px',
                    color: 'var(--text-primary)',
                    margin: 0,
                    overflow: 'auto',
                    border: '1px solid var(--card-border)',
                  }}
                >{`wallet.send(usdcToken, {
  value: 500000000n,   // 500 USDC (6 decimals)
  to: "0x7a2f...",
  chain: "ethereum"
});

// ⚠ Who checks sanctions?
// ⚠ Who requires approval?
// ⚠ Who decides the chain?`}</pre>
              </div>

              <div>
                <p className="font-sans font-semibold" style={{ fontSize: '13.5px', color: 'var(--text-muted)', margin: '0 0 0.6rem', letterSpacing: '0.04em' }}>
                  WHAT AGENTS DECLARE WITH BASTION
                </p>
                <pre
                  className="font-mono"
                  style={{
                    fontSize: '13px',
                    lineHeight: 1.6,
                    background: 'var(--accent-emphasis)',
                    padding: '1.4rem',
                    borderRadius: '8px',
                    color: 'var(--text-primary)',
                    margin: 0,
                    overflow: 'auto',
                    border: '1px solid var(--accent)',
                  }}
                >{`intent: transfer
asset: USDC
amount: 500
recipient: 0x...
requirements:
  - humanApproval
  - sanctionsCheck
  - maxRisk: medium
  - settlement: ethereum`}</pre>
              </div>
            </div>

            <p
              className="font-sans mt-8"
              style={{ fontSize: '14px', lineHeight: 1.65, color: 'var(--text-muted)', maxWidth: '52ch', margin: '2.5rem 0 0' }}
            >
              TrustIntent keeps agent frameworks focused on producing{' '}
              <em>what</em> should happen, while Bastion owns <em>how</em> it
              is carried out under programmable trust policies - identity,
              simulation, routing, settlement, audit.
            </p>
          </div>
        </section>

        {/* ── Case studies ── */}
        <section className="band-stone">
          <div className="max-w-[1180px] mx-auto px-6 sm:px-10 py-24 sm:py-32">
            <p className="band-kicker">How it is used</p>
            <h2
              className="font-display"
              style={{ fontSize: 'clamp(1.8rem, 3.2vw, 2.5rem)', letterSpacing: '-0.03em', margin: '0 0 4rem' }}
            >
              Where a runtime earns its place
            </h2>

            <div
              className="grid gap-x-12 gap-y-14"
              style={{ gridTemplateColumns: 'repeat(auto-fit, minmax(min(100%, 280px), 1fr))' }}
            >
              {CASES.map((c, i) => (
                <div key={c.title}>
                  <BastionMark size={34} color="var(--text-primary)" />
                  <h3
                    className="font-display"
                    style={{ fontSize: '1.32rem', letterSpacing: '-0.024em', margin: '1.6rem 0 0.9rem' }}
                  >
                    {c.title}
                  </h3>
                  <p className="font-sans" style={{ fontSize: '14.5px', lineHeight: 1.68, color: 'var(--text-secondary)', margin: 0 }}>
                    {c.body}
                  </p>
                  <p className="font-sans" style={{ fontSize: '14.5px', lineHeight: 1.68, color: 'var(--text-muted)', margin: '1.2rem 0 0' }}>
                    {c.best}
                  </p>
                  <span className="sr-only">{i}</span>
                </div>
              ))}
            </div>
          </div>
        </section>

        {/* ── Foundations ── */}
        <section>
          <div className="max-w-[1180px] mx-auto px-6 sm:px-10 py-24 sm:py-32">
            <p className="band-kicker">Foundation</p>
            <h2
              className="font-display"
              style={{
                fontSize: 'clamp(1.8rem, 3.2vw, 2.5rem)',
                letterSpacing: '-0.03em',
                lineHeight: 1.14,
                margin: '0 0 4rem',
                maxWidth: '18ch',
              }}
            >
              Built on standards, not around them
            </h2>

            <div
              className="grid gap-x-12 gap-y-12"
              style={{ gridTemplateColumns: 'repeat(auto-fit, minmax(min(100%, 230px), 1fr))' }}
            >
              {FOUNDATIONS.map((f, i) => (
                <div key={f.title} style={{ borderLeft: '1px solid var(--border)', paddingLeft: '1.6rem' }}>
                  <FoundationIcon index={i} />
                  <h3
                    className="font-display"
                    style={{ fontSize: '1.18rem', letterSpacing: '-0.022em', margin: '1.3rem 0 0.7rem' }}
                  >
                    {f.title}
                  </h3>
                  <p className="font-sans" style={{ fontSize: '14px', lineHeight: 1.65, color: 'var(--text-secondary)', margin: 0 }}>
                    {f.body}
                  </p>
                </div>
              ))}
            </div>
          </div>
        </section>

        {/* ── Status ──
          * Published as-is. A runtime asking to be trusted should be honest about
          * what it has not finished. */}
        <section id="status" style={{ borderTop: '1px solid var(--border)' }}>
          <div className="max-w-[1180px] mx-auto px-6 sm:px-10 py-24 sm:py-28">
            <p className="band-kicker">Status</p>
            <h2
              className="font-display"
              style={{ fontSize: 'clamp(1.7rem, 3vw, 2.3rem)', letterSpacing: '-0.03em', margin: 0, maxWidth: '22ch' }}
            >
              What is finished, and what is not
            </h2>

            <ul
              className="grid gap-x-14"
              style={{
                gridTemplateColumns: 'repeat(auto-fit, minmax(min(100%, 320px), 1fr))',
                listStyle: 'none',
                padding: 0,
                margin: '3.5rem 0 0',
              }}
            >
              {STATUS.map(s => (
                <li
                  key={s.label}
                  className="flex items-baseline justify-between gap-6 py-4"
                  style={{ borderBottom: '1px solid var(--border)' }}
                >
                  <span className="font-sans" style={{ fontSize: '14.5px', color: 'var(--text-secondary)' }}>
                    {s.label}
                  </span>
                  <span
                    className="font-mono uppercase"
                    style={{ fontSize: '10px', letterSpacing: '0.1em', color: STATE_COLOR[s.state], whiteSpace: 'nowrap' }}
                  >
                    {STATE_COPY[s.state]}
                  </span>
                </li>
              ))}
            </ul>
          </div>
        </section>

        {/* ── Call to action ── */}
        <section className="band-ink">
          <div className="max-w-[1180px] mx-auto px-6 sm:px-10 py-24 sm:py-32">
            <h2
              className="band-title font-display"
              style={{
                fontSize: 'clamp(2rem, 4.4vw, 3.4rem)',
                lineHeight: 1.1,
                letterSpacing: '-0.032em',
                margin: 0,
                maxWidth: '16ch',
              }}
            >
              Ready to ship an agent you can prove?
            </h2>
            <p className="band-body font-sans mt-6" style={{ fontSize: '15.5px', lineHeight: 1.7, maxWidth: '44ch' }}>
              Integrate the SDK, set a policy, and route signing through the runtime.
              Open source under Apache 2.0.
            </p>
            <div className="mt-10 flex flex-wrap items-center gap-7">
              <Link
                to="/integrate"
                className="font-sans no-underline"
                style={{
                  fontSize: '14px',
                  fontWeight: 500,
                  padding: '0.8rem 1.7rem',
                  borderRadius: '999px',
                  background: 'var(--accent)',
                  color: 'oklch(0.165 0.006 84.57)',
                  textDecoration: 'none',
                }}
              >
                Integrate your agent
              </Link>
              <Link
                to="/docs"
                className="font-sans no-underline"
                style={{ fontSize: '14px', color: 'oklch(0.79 0.005 84.57)', textDecoration: 'none' }}
              >
                Read the docs ↗
              </Link>
            </div>
          </div>
        </section>

        {/* ── Standards ──
          * The reference puts a wall of employer logos here. Those are other
          * companies' marks and other people's credibility, so this beat carries
          * the composition Bastion can actually claim: the primitives it builds on. */}
        <section id="standards" className="band-orange">
          <div className="max-w-[1180px] mx-auto px-6 sm:px-10 py-24 sm:py-32">
            <p className="band-kicker">Composed, not reinvented</p>
            <h2
              className="band-title font-display"
              style={{
                fontSize: 'clamp(1.8rem, 3.4vw, 2.7rem)',
                lineHeight: 1.14,
                letterSpacing: '-0.03em',
                margin: 0,
                maxWidth: '20ch',
              }}
            >
              Bastion orchestrates the ecosystem's trust primitives
            </h2>
            <p className="band-body font-sans mt-6" style={{ fontSize: '15.5px', lineHeight: 1.7, maxWidth: '48ch' }}>
              It does not replace them. Each standard keeps doing its job; the
              runtime coordinates them and adds the decision layer on top.
            </p>

            <div
              className="grid gap-x-12 gap-y-8 mt-16"
              style={{ gridTemplateColumns: 'repeat(auto-fit, minmax(min(100%, 260px), 1fr))' }}
            >
              {STANDARDS.map(s => (
                <div key={s.id} style={{ borderTop: '1px solid oklch(0.24 0.03 44 / 0.28)', paddingTop: '1.1rem' }}>
                  <p
                    className="font-mono"
                    style={{ fontSize: '13px', margin: '0 0 0.5rem', color: 'oklch(0.165 0.006 84.57)' }}
                  >
                    {s.id}
                  </p>
                  <p className="band-body font-sans" style={{ fontSize: '14px', lineHeight: 1.6, margin: 0 }}>
                    {s.adds}
                  </p>
                </div>
              ))}
            </div>
          </div>
        </section>
      </main>

      {/* ── Footer ── */}
      <footer className="band-ink relative overflow-hidden">
        <div className="relative z-10 max-w-[1180px] mx-auto px-6 sm:px-10 pt-20">
          <div
            className="grid gap-x-12 gap-y-12 pb-16"
            style={{ gridTemplateColumns: 'repeat(auto-fit, minmax(min(100%, 190px), 1fr))' }}
          >
            <div style={{ maxWidth: '32ch' }}>
              <BastionLockup size={22} color="oklch(0.955 0.004 84.57)" markColor="var(--accent)" />
              <p className="band-body font-sans" style={{ fontSize: '13.5px', lineHeight: 1.7, margin: '1.2rem 0 0' }}>
                A programmable trust runtime for autonomous systems. Open-source community project. Built by ZKOS
                Labs.
              </p>
            </div>

            {[
              {
                head: 'Product',
                links: [
                  ['Runtime', '#runtime'],
                  ['Standards', '#standards'],
                  ['Status', '#status'],
                  ['Integrate', '/integrate'],
                ],
              },
              {
                head: 'Developers',
                links: [
                  ['Documentation', '/docs'],
                  ['Quickstart', '/docs/quickstart'],
                  ['API reference', '/docs/api-reference'],
                  ['MCP server', '/docs/mcp'],
                ],
              },
              {
                head: 'Project',
                links: [
                  ['GitHub', 'https://github.com/zkos-labs/bastion'],
                  ['Donate', 'https://github.com/sponsors/zkos-labs'],
                  ['X', 'https://x.com/BastionAgntque'],
                  ['Discord', 'https://discord.gg/hXVFB2Tz2t'],
                ],
              },
            ].map(col => (
              <div key={col.head}>
                <p className="band-kicker" style={{ margin: '0 0 1.25rem' }}>
                  {col.head}
                </p>
                <ul style={{ listStyle: 'none', margin: 0, padding: 0 }}>
                  {col.links.map(([label, href]) => (
                    <li key={label} style={{ marginBottom: '0.7rem' }}>
                      {href.startsWith('/') ? (
                        <Link
                          to={href}
                          className="font-sans no-underline nav-link"
                          style={{ fontSize: '13.5px', color: 'oklch(0.79 0.005 84.57)' }}
                        >
                          {label}
                        </Link>
                      ) : (
                        <a
                          href={href}
                          target={href.startsWith('http') ? '_blank' : undefined}
                          rel={href.startsWith('http') ? 'noopener noreferrer' : undefined}
                          className="font-sans no-underline nav-link"
                          style={{ fontSize: '13.5px', color: 'oklch(0.79 0.005 84.57)' }}
                        >
                          {label}
                        </a>
                      )}
                    </li>
                  ))}
                </ul>
              </div>
            ))}
          </div>

          <div
            className="flex flex-wrap items-baseline justify-between gap-4 py-7"
            style={{ borderTop: '1px solid oklch(0.95 0.005 84.57 / 0.11)' }}
          >
            <span className="font-sans" style={{ fontSize: '12.5px', color: 'oklch(0.6 0.006 84.57)' }}>
              © 2026 ZKOS Labs. Apache 2.0.
            </span>
            <span className="font-sans" style={{ fontSize: '12.5px', color: 'oklch(0.6 0.006 84.57)' }}>
              Alpha - not audited for mainnet use.
            </span>
          </div>
        </div>

        {/* Signature wordmark: flush to the bottom edge, room above so no cap is
            shaved, and on the layer above everything else. */}
        <div className="relative z-10 w-full overflow-hidden" style={{ marginBottom: '-0.13em' }} aria-hidden="true">
          <div
            className="font-display select-none"
            style={{
              fontSize: 'clamp(3.4rem, 16.5vw, 14rem)',
              lineHeight: 0.9,
              letterSpacing: '0.02em',
              fontWeight: 500,
              textAlign: 'center',
              color: 'oklch(0.955 0.004 84.57)',
              opacity: 0.07,
              paddingTop: '0.14em',
              whiteSpace: 'nowrap',
            }}
          >
            BASTION
          </div>
        </div>
      </footer>
    </div>
  );
}
