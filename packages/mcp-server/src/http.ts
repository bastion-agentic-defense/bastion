/**
 * MCP HTTP Server - SSE transport for browser-native access.
 *
 * Bastion is a non-profit trust runtime, like Grafana.
 * The core infrastructure is free and open source (Apache 2.0).
 * Backend API calls are optionally paid via USDT/USDC to cover
 * infrastructure costs. No tokens, no treasuries, no paywalls.
 *
 * Usage:
 *   BASTION_SIDECAR_URL=https://bastion-agentique.fly.dev/ \
 *   tsx src/http.ts
 */

import { SSEServerTransport } from '@modelcontextprotocol/sdk/server/sse.js';
import { McpServer } from '@modelcontextprotocol/sdk/server/mcp.js';
import { createToolDefinitions, SIDECAR_URL as SIDECAR } from './index.js';
import { getPricingTable } from './pricing.js';
import { createServer } from 'http';
import { URL } from 'url';

const PORT = parseInt(process.env.BASTION_MCP_PORT || '3001', 10);
const SIDECAR_URL = SIDECAR;

const server = new McpServer({
  name: 'bastion-mcp',
  version: '0.3.0',
  description: 'Bastion Programmable Trust Runtime - simulate, audit, and secure every transaction before signing. Non-profit, open source.',
});

createToolDefinitions(server);

const transports = new Map<string, SSEServerTransport>();

const httpServer = createServer(async (req, res) => {
  const url = new URL(req.url || '/', `http://${req.headers.host}`);

  res.setHeader('Access-Control-Allow-Origin', '*');
  res.setHeader('Access-Control-Allow-Methods', 'GET, POST, DELETE, OPTIONS');
  res.setHeader('Access-Control-Allow-Headers', 'Content-Type, Authorization, X-Agent-Id');

  if (req.method === 'OPTIONS') {
    res.writeHead(204);
    res.end();
    return;
  }

  if (req.method === 'GET' && url.pathname === '/mcp/health') {
    res.writeHead(200, { 'Content-Type': 'application/json' });
    res.end(JSON.stringify({
      status: 'ok',
      transport: 'sse',
      port: PORT,
      sidecar: SIDECAR_URL,
      version: '0.3.0',
      non_profit: true,
    }));
    return;
  }

  if (req.method === 'GET' && url.pathname === '/mcp/pricing') {
    const pricing = getPricingTable();
    res.writeHead(200, { 'Content-Type': 'application/json' });
    res.end(JSON.stringify({
      notice: 'Bastion is a non-profit trust runtime. Free tier resets on 1st of each month. Backend API calls are optionally paid via USDT/USDC to cover infrastructure costs.',
      tools: pricing,
    }));
    return;
  }

  if (req.method === 'GET' && url.pathname === '/mcp/sse') {
    const transport = new SSEServerTransport('/mcp/messages', res);
    transports.set(transport.sessionId, transport);
    res.on('close', () => transports.delete(transport.sessionId));
    await server.connect(transport);
    return;
  }

  if (req.method === 'POST' && url.pathname === '/mcp/messages') {
    const sessionId = url.searchParams.get('sessionId') || '';
    const transport = transports.get(sessionId);
    if (!transport) {
      res.writeHead(400, { 'Content-Type': 'application/json' });
      res.end(JSON.stringify({ error: 'No active SSE session. Connect via GET /mcp/sse first.' }));
      return;
    }
    await transport.handlePostMessage(req, res);
    return;
  }

  if (req.method === 'POST' && url.pathname === '/mcp') {
    res.writeHead(302, { Location: '/mcp/sse' });
    res.end();
    return;
  }

  res.writeHead(404, { 'Content-Type': 'application/json' });
  res.end(JSON.stringify({ error: 'Not found. Use GET /mcp/sse for SSE connection, POST /mcp/messages for MCP messages.' }));
});

httpServer.listen(PORT, '0.0.0.0', () => {
  console.log(`[bastion-mcp] SSE MCP server listening on 0.0.0.0:${PORT}`);
  console.log(`[bastion-mcp] SSE endpoint: http://localhost:${PORT}/mcp/sse`);
  console.log(`[bastion-mcp] Messages endpoint: http://localhost:${PORT}/mcp/messages`);
  console.log(`[bastion-mcp] Sidecar: ${SIDECAR_URL}`);
  console.log(`[bastion-mcp] Bastion is a non-profit trust runtime. Backend API calls optional paid via USDT/USDC.`);
});
