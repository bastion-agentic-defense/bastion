/**
 * Markdown-for-Agents: content-negotiation edge function.
 *
 * When a request includes `Accept: text/markdown`, serve a markdown version of
 * the page. Browsers without the header get the normal HTML SPA.
 *
 * Path          Markdown variant
 * /          →  /index.md
 * /integrate →  /integrate.md
 * /dashboard →  /dashboard.md
 *
 * Registered on those three routes via the exported `config.path` below, so it
 * never intercepts static asset requests.
 */
import type { Context } from "@netlify/edge-functions";

const MARKDOWN_VARIANTS: Record<string, string> = {
  "/": "/index.md",
  "/integrate": "/integrate.md",
  "/dashboard": "/dashboard.md",
};

export default async function handler(request: Request, context: Context) {
  const accept = request.headers.get("accept") || "";
  if (!accept.includes("text/markdown")) {
    return context.next();
  }

  const url = new URL(request.url);
  const variant = MARKDOWN_VARIANTS[url.pathname];
  if (!variant) {
    return context.next();
  }

  url.pathname = variant;
  const response = await context.next(new Request(url.toString(), request));

  // Markdown variant missing (non-200): fall through to the normal HTML page.
  if (response.status !== 200) {
    return context.next();
  }

  const headers = new Headers(response.headers);
  headers.set("Content-Type", "text/markdown; charset=UTF-8");
  headers.set("X-Markdown-Tokens", response.headers.get("content-length") || "0");
  return new Response(response.body, { status: 200, headers });
}

export const config = { path: ["/", "/integrate", "/dashboard"] };
