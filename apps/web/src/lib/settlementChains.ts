// Canonical multichain settlement list, matching the SDK's `Settlement`
// type (packages/sdk/src/execute.ts) and the sidecar's simulate endpoints.
// Distinct from chains.ts, which drives the wagmi/RainbowKit wallet config
// (network selection for signing) rather than display/filtering.

export interface SettlementChain {
  id: string;
  label: string;
  color: string;
}

export const SETTLEMENT_CHAINS: SettlementChain[] = [
  { id: 'ethereum', label: 'Ethereum', color: '#627EEA' },
  { id: 'base', label: 'Base', color: '#0052FF' },
  { id: 'celo', label: 'Celo', color: '#FCFF52' },
  { id: 'zksync', label: 'zkSync', color: '#8C8DFC' },
  { id: 'robinhood', label: 'Robinhood', color: '#00C805' },
  { id: 'monad', label: 'Monad', color: '#836EF9' },
  { id: 'polygon', label: 'Polygon', color: '#8247E5' },
  { id: 'arbitrum', label: 'Arbitrum', color: '#28A0F0' },
  { id: 'solana', label: 'Solana', color: '#9945FF' },
];

export const SETTLEMENT_CHAIN_IDS = SETTLEMENT_CHAINS.map((c) => c.id);

/** Extract the chain segment from a Bastion DID (`did:bastion:{chain}:{pubkey}`). */
export function chainFromDid(did: string | null | undefined): string {
  if (!did) return 'unknown';
  const parts = did.split(':');
  return parts.length >= 3 ? parts[2] : 'unknown';
}

export function chainLabel(id: string): string {
  return SETTLEMENT_CHAINS.find((c) => c.id === id)?.label ?? id;
}

export function chainColor(id: string): string {
  return SETTLEMENT_CHAINS.find((c) => c.id === id)?.color ?? '#71717a';
}
