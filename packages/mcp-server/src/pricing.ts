/**
 * Tool pricing definitions for the MCP HTTP server.
 * Exposes GET /mcp/pricing for agents to discover tool costs.
 * All prices in USD. Backend API calls are optional paid via USDT/USDC.
 */

import { TOOL_PRICES } from './credits.js';

export interface PricingEntry {
  tool: string;
  free_per_month: number;
  price_usd: number;
  category: 'free' | 'paid';
  description: string;
}

const PRICING_TABLE: Record<string, PricingEntry> = {
  bastion_simulate_transaction: {
    tool: 'bastion_simulate_transaction',
    free_per_month: 100,
    price_usd: 0.10,
    category: 'paid',
    description: 'Simulate a transaction through Bastion firewall and policy engine',
  },
  bastion_override_block: {
    tool: 'bastion_override_block',
    free_per_month: 10,
    price_usd: 1.00,
    category: 'paid',
    description: 'Human-in-the-loop override for a blocked transaction',
  },
  bastion_update_policy: {
    tool: 'bastion_update_policy',
    free_per_month: 5,
    price_usd: 5.00,
    category: 'paid',
    description: 'Update firewall policy (allowlists, caps, rate limits)',
  },
  bastion_circuit_breaker_toggle: {
    tool: 'bastion_circuit_breaker_toggle',
    free_per_month: 3,
    price_usd: 10.00,
    category: 'paid',
    description: 'Engage or disengage fleet-wide circuit breaker',
  },
};

export function getPricingTable(): PricingEntry[] {
  const entries: PricingEntry[] = [];
  for (const [tool] of Object.entries(TOOL_PRICES)) {
    if (PRICING_TABLE[tool]) {
      entries.push(PRICING_TABLE[tool]);
    }
  }
  const freeTools = [
    'bastion_get_policy',
    'bastion_get_audit_logs',
    'bastion_get_audit_stats',
    'bastion_get_pending_approvals',
    'bastion_circuit_breaker_status',
    'bastion_resolve_did',
    'bastion_list_cases',
    'bastion_get_token_balances',
  ];
  for (const tool of freeTools) {
    entries.push({
      tool,
      free_per_month: Infinity,
      price_usd: 0,
      category: 'free',
      description: 'Read-only operation',
    });
  }
  return entries;
}
