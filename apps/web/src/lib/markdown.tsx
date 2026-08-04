import type { ReactNode } from 'react';

/* ─────────────────────────────────────────────────────────────────────────────
 * A small, deliberate CommonMark subset.
 *
 * The docs in `docs/*.md` are the source of truth and are rendered directly, so
 * the site and the repository cannot drift apart. Only the constructs those
 * files actually use are supported: headings, paragraphs, fenced code, lists,
 * tables, blockquotes, and inline code / bold / links.
 *
 * This is not a general markdown parser and should not be pointed at untrusted
 * input. It renders through React elements (never `dangerouslySetInnerHTML`), so
 * text is escaped, but the grammar it accepts is narrow by design.
 * ──────────────────────────────────────────────────────────────────────────── */

export interface Heading {
  id: string;
  text: string;
  level: 2 | 3;
}

export function slugify(text: string): string {
  return text
    .toLowerCase()
    .replace(/[^\w\s-]/g, '')
    .trim()
    .replace(/\s+/g, '-');
}

/** Headings used to build the in-page table of contents. */
export function extractHeadings(src: string): Heading[] {
  const out: Heading[] = [];
  let inFence = false;

  for (const line of src.split('\n')) {
    if (line.startsWith('```')) {
      inFence = !inFence;
      continue;
    }
    if (inFence) continue;

    const m = /^(##|###)\s+(.*)$/.exec(line);
    if (m) {
      const text = m[2].trim();
      out.push({ id: slugify(text), text, level: m[1].length as 2 | 3 });
    }
  }
  return out;
}

/** Inline: `code`, **bold**, [text](href). */
function inline(text: string, keyPrefix: string): ReactNode[] {
  const nodes: ReactNode[] = [];
  const pattern = /(`[^`]+`)|(\*\*[^*]+\*\*)|(\[[^\]]+\]\([^)]+\))/g;
  let last = 0;
  let m: RegExpExecArray | null;
  let i = 0;

  while ((m = pattern.exec(text)) !== null) {
    if (m.index > last) nodes.push(text.slice(last, m.index));
    const token = m[0];
    const key = `${keyPrefix}-i${i++}`;

    if (token.startsWith('`')) {
      nodes.push(
        <code key={key} className="md-code">
          {token.slice(1, -1)}
        </code>,
      );
    } else if (token.startsWith('**')) {
      nodes.push(
        <strong key={key} style={{ color: 'var(--text-primary)', fontWeight: 600 }}>
          {token.slice(2, -2)}
        </strong>,
      );
    } else {
      const linkMatch = /^\[([^\]]+)\]\(([^)]+)\)$/.exec(token);
      if (linkMatch) {
        const [, label, href] = linkMatch;
        const external = href.startsWith('http');
        nodes.push(
          <a
            key={key}
            href={href}
            target={external ? '_blank' : undefined}
            rel={external ? 'noopener noreferrer' : undefined}
            className="md-link"
          >
            {label}
          </a>,
        );
      } else {
        nodes.push(token);
      }
    }
    last = m.index + token.length;
  }

  if (last < text.length) nodes.push(text.slice(last));
  return nodes;
}

function splitRow(row: string): string[] {
  return row
    .replace(/^\||\|$/g, '')
    .split('|')
    .map(c => c.trim());
}

export function renderMarkdown(src: string): ReactNode[] {
  const lines = src.split('\n');
  const out: ReactNode[] = [];
  let i = 0;
  let key = 0;

  while (i < lines.length) {
    const line = lines[i];

    if (!line.trim()) {
      i++;
      continue;
    }

    // Fenced code
    if (line.startsWith('```')) {
      const lang = line.slice(3).trim();
      const body: string[] = [];
      i++;
      while (i < lines.length && !lines[i].startsWith('```')) {
        body.push(lines[i]);
        i++;
      }
      i++; // closing fence
      out.push(
        <pre key={`k${key++}`} className="md-pre">
          {lang && <span className="md-pre-lang">{lang}</span>}
          <code>{body.join('\n')}</code>
        </pre>,
      );
      continue;
    }

    // Headings
    const h = /^(#{1,3})\s+(.*)$/.exec(line);
    if (h) {
      const text = h[2].trim();
      const id = slugify(text);
      if (h[1].length === 1) {
        out.push(
          <h1 key={`k${key++}`} id={id} className="md-h1 font-display">
            {text}
          </h1>,
        );
      } else if (h[1].length === 2) {
        out.push(
          <h2 key={`k${key++}`} id={id} className="md-h2 font-display">
            {text}
          </h2>,
        );
      } else {
        out.push(
          <h3 key={`k${key++}`} id={id} className="md-h3 font-mono">
            {text}
          </h3>,
        );
      }
      i++;
      continue;
    }

    // Blockquote
    if (line.startsWith('> ')) {
      const body: string[] = [];
      while (i < lines.length && lines[i].startsWith('>')) {
        body.push(lines[i].replace(/^>\s?/, ''));
        i++;
      }
      out.push(
        <blockquote key={`k${key++}`} className="md-quote">
          {inline(body.join(' '), `q${key}`)}
        </blockquote>,
      );
      continue;
    }

    // Table
    if (line.trim().startsWith('|') && /^\s*\|[\s:|-]+\|\s*$/.test(lines[i + 1] ?? '')) {
      const head = splitRow(line);
      i += 2;
      const rows: string[][] = [];
      while (i < lines.length && lines[i].trim().startsWith('|')) {
        rows.push(splitRow(lines[i]));
        i++;
      }
      out.push(
        <div key={`k${key++}`} className="md-table-wrap">
          <table className="md-table">
            <thead>
              <tr>
                {head.map((c, ci) => (
                  <th key={ci} scope="col">
                    {inline(c, `th${ci}`)}
                  </th>
                ))}
              </tr>
            </thead>
            <tbody>
              {rows.map((r, ri) => (
                <tr key={ri}>
                  {r.map((c, ci) => (
                    <td key={ci}>{inline(c, `td${ri}-${ci}`)}</td>
                  ))}
                </tr>
              ))}
            </tbody>
          </table>
        </div>,
      );
      continue;
    }

    // Unordered list
    if (/^[-*]\s+/.test(line)) {
      const items: string[] = [];
      while (i < lines.length && /^[-*]\s+/.test(lines[i])) {
        items.push(lines[i].replace(/^[-*]\s+/, ''));
        i++;
      }
      out.push(
        <ul key={`k${key++}`} className="md-ul">
          {items.map((it, ii) => (
            <li key={ii}>{inline(it, `li${ii}`)}</li>
          ))}
        </ul>,
      );
      continue;
    }

    // Ordered list
    if (/^\d+\.\s+/.test(line)) {
      const items: string[] = [];
      while (i < lines.length && /^\d+\.\s+/.test(lines[i])) {
        items.push(lines[i].replace(/^\d+\.\s+/, ''));
        i++;
      }
      out.push(
        <ol key={`k${key++}`} className="md-ol">
          {items.map((it, ii) => (
            <li key={ii}>{inline(it, `oli${ii}`)}</li>
          ))}
        </ol>,
      );
      continue;
    }

    // Paragraph - consume until a blank line or a block-level construct.
    const para: string[] = [];
    while (
      i < lines.length &&
      lines[i].trim() &&
      !lines[i].startsWith('#') &&
      !lines[i].startsWith('```') &&
      !lines[i].startsWith('>') &&
      !lines[i].trim().startsWith('|') &&
      !/^[-*]\s+/.test(lines[i]) &&
      !/^\d+\.\s+/.test(lines[i])
    ) {
      para.push(lines[i].trim());
      i++;
    }
    out.push(
      <p key={`k${key++}`} className="md-p">
        {inline(para.join(' '), `p${key}`)}
      </p>,
    );
  }

  return out;
}
