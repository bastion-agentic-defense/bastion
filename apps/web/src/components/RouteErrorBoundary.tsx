import { Component } from 'react';
import type { ErrorInfo, ReactNode } from 'react';

interface Props {
  children: ReactNode;
}

interface State {
  error: Error | null;
}

/**
 * Catches render-time crashes anywhere below it and renders a readable failure
 * state instead of an empty document.
 *
 * Without this, a single throwing hook (for example `useWallet()` mounted
 * outside a `WalletProvider`) unmounts the whole tree and the route paints as a
 * blank page with no visible explanation.
 */
export class RouteErrorBoundary extends Component<Props, State> {
  state: State = { error: null };

  static getDerivedStateFromError(error: Error): State {
    return { error };
  }

  componentDidCatch(error: Error, info: ErrorInfo) {
    console.error('[Bastion] route crashed:', error, info.componentStack);
  }

  render() {
    const { error } = this.state;
    if (!error) return this.props.children;

    return (
      <div
        role="alert"
        className="min-h-screen flex items-center justify-center px-6"
        style={{ background: 'var(--bg)', color: 'var(--text-primary)' }}
      >
        <div className="max-w-md">
          <h1 className="font-serif text-3xl mb-3">This page failed to load.</h1>
          <p className="font-sans text-sm mb-6" style={{ color: 'var(--text-muted)' }}>
            An unexpected error stopped the page from rendering. The details are in the
            browser console.
          </p>
          <pre
            className="font-mono text-xs p-4 rounded-lg overflow-x-auto mb-6"
            style={{ background: 'var(--bg-subtle)', border: '1px solid var(--card-border)' }}
          >
            {error.message}
          </pre>
          <a
            href="/"
            className="font-sans text-sm underline underline-offset-4"
            style={{ color: 'var(--text-primary)' }}
          >
            Return home
          </a>
        </div>
      </div>
    );
  }
}
