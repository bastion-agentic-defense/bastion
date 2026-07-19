import { useEffect, useMemo } from 'react';
import { Link, useParams, Navigate } from 'react-router-dom';

import quickstartSrc from '../../../../docs/quickstart.md?raw';
import apiReferenceSrc from '../../../../docs/api-reference.md?raw';
import mcpSrc from '../../../../docs/mcp.md?raw';

import { renderMarkdown, extractHeadings } from '../lib/markdown';
import { BastionLockup } from '../components/BastionMark';

/* The markdown files in `docs/` are the source of truth. They are rendered here
 * directly, so the published docs and the repository cannot drift apart. */
const PAGES = [
  { slug: 'quickstart', title: 'Quickstart', blurb: 'Ten minutes to a guarded agent', src: quickstartSrc },
  { slug: 'api-reference', title: 'API reference', blurb: 'Every endpoint and error', src: apiReferenceSrc },
  { slug: 'mcp', title: 'MCP server', blurb: 'Runtime tools for coding agents', src: mcpSrc },
];

function DocsNav() {
  return (
    <nav
      className="fixed top-0 inset-x-0 z-50 glass"
      style={{ borderBottom: '1px solid var(--border)' }}
      aria-label="Main"
    >
      <div className="max-w-[1180px] mx-auto px-6 sm:px-10 h-[68px] flex items-center justify-between gap-6">
        <div className="flex items-center gap-3">
          <Link to="/" className="no-underline" style={{ color: 'var(--text-primary)' }} aria-label="Bastion, home">
            <BastionLockup size={23} markColor="var(--accent)" />
          </Link>
          <span className="font-mono" style={{ fontSize: '11px', color: 'var(--text-faint)' }}>
            docs
          </span>
        </div>
        <div className="flex items-center gap-8">
          <a
            href="https://github.com/zkos-labs/bastion"
            target="_blank"
            rel="noopener noreferrer"
            className="font-sans no-underline nav-link"
            style={{ fontSize: '13.5px', color: 'var(--text-muted)' }}
          >
            GitHub ↗
          </a>
          <Link to="/integrate" className="btn-primary no-underline" style={{ textDecoration: 'none' }}>
            Integrate
          </Link>
        </div>
      </div>
    </nav>
  );
}

export default function Docs() {
  const { slug } = useParams();
  const active = slug ?? 'quickstart';
  const page = PAGES.find(p => p.slug === active);

  const headings = useMemo(() => (page ? extractHeadings(page.src) : []), [page]);
  const body = useMemo(() => (page ? renderMarkdown(page.src) : []), [page]);

  // A fresh document starts at the top, not wherever the last one was scrolled to.
  useEffect(() => {
    window.scrollTo(0, 0);
  }, [active]);

  if (!page) return <Navigate to="/docs/quickstart" replace />;

  return (
    <div style={{ background: 'var(--bg)', color: 'var(--text-primary)', minHeight: '100vh' }}>
      <DocsNav />

      <div
        className="max-w-[1180px] mx-auto px-6 sm:px-10"
        style={{ paddingTop: 'calc(68px + 3.5rem)', paddingBottom: '6rem' }}
      >
        <div className="docs-shell">
          {/* Documents */}
          <aside className="docs-aside" aria-label="Documentation">
            <p
              className="font-mono uppercase"
              style={{ fontSize: '10px', letterSpacing: '0.12em', color: 'var(--text-faint)', margin: '0 0 1.1rem' }}
            >
              Documentation
            </p>
            <ul style={{ listStyle: 'none', margin: 0, padding: 0 }}>
              {PAGES.map(p => {
                const current = p.slug === active;
                return (
                  <li key={p.slug} style={{ marginBottom: '1.1rem' }}>
                    <Link
                      to={`/docs/${p.slug}`}
                      className="no-underline"
                      aria-current={current ? 'page' : undefined}
                      style={{
                        display: 'block',
                        fontSize: '14px',
                        fontWeight: current ? 600 : 400,
                        color: current ? 'var(--text-primary)' : 'var(--text-secondary)',
                      }}
                    >
                      {p.title}
                      <span
                        className="font-sans"
                        style={{
                          display: 'block',
                          fontSize: '12px',
                          color: 'var(--text-faint)',
                          fontWeight: 400,
                          marginTop: '2px',
                        }}
                      >
                        {p.blurb}
                      </span>
                    </Link>
                  </li>
                );
              })}
            </ul>
          </aside>

          {/* Body */}
          <article className="docs-body">{body}</article>

          {/* On this page */}
          <aside className="docs-toc" aria-label="On this page">
            <p
              className="font-mono uppercase"
              style={{ fontSize: '10px', letterSpacing: '0.12em', color: 'var(--text-faint)', margin: '0 0 1.1rem' }}
            >
              On this page
            </p>
            <ul style={{ listStyle: 'none', margin: 0, padding: 0 }}>
              {headings.map(h => (
                <li key={h.id} style={{ marginBottom: '0.6rem', paddingLeft: h.level === 3 ? '0.9rem' : 0 }}>
                  <a
                    href={`#${h.id}`}
                    className="font-sans no-underline nav-link"
                    style={{ fontSize: h.level === 3 ? '12.5px' : '13px', color: 'var(--text-muted)' }}
                  >
                    {h.text}
                  </a>
                </li>
              ))}
            </ul>
          </aside>
        </div>
      </div>
    </div>
  );
}
