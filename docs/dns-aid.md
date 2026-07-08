# DNS for AI Discovery (DNS-AID)

DNS-AID lets agents discover Bastion's entrypoints straight from DNS, before ever fetching a web
page. It publishes ServiceMode **SVCB/HTTPS** records (RFC 9460) under the `_agents` namespace of
`bastionagentique.com`, and the discovery zone is signed with **DNSSEC** so validating resolvers
return authenticated answers.

These records live at the DNS provider, **not in this repo** — this file is the source of truth for
what to publish. Apply them in the DNS dashboard for `bastionagentique.com`, then verify with the
commands below.

Reference: draft-mozleywilliams-dnsop-dnsaid, RFC 9460 (SVCB/HTTPS records).

## Records to publish

```dns
; ── Index entrypoint ────────────────────────────────────────────────
; Points agents at the agent-ready site (Link headers, /.well-known, auth.md).
_index._agents.bastionagentique.com. 3600 IN SVCB 1 bastionagentique.com. (
    alpn="h2,http/1.1" port=443 mandatory=alpn,port )

; ── Agent-to-agent (A2A) endpoint ───────────────────────────────────
; The Bastion sidecar firewall API.
_a2a._agents.bastionagentique.com. 3600 IN SVCB 1 bastion-agentique.fly.dev. (
    alpn="a2a" port=443 mandatory=alpn,port )
```

Notes:
- **ServiceMode** = priority `1` (a non-zero priority). The field after it is the target host.
- `alpn` + `port` are the minimum connection params; `mandatory=alpn,port` marks them required.
- If your provider's UI does not expose `mandatory`, publish at least `alpn` and `port`.
- Adjust the A2A target if the API moves off `bastion-agentique.fly.dev` (e.g. to
  `api.bastionagentique.com`). Confirm final targets before publishing.

## DNSSEC

Enable DNSSEC for `bastionagentique.com` so the `_agents` records are authenticated:

1. In the DNS provider, turn on DNSSEC for the zone (it generates ZSK/KSK and signs records).
2. Copy the generated **DS record** into the registrar for `bastionagentique.com` to build the chain
   of trust. (If registrar and DNS provider are the same, this is usually one toggle.)
3. Wait for propagation, then confirm the chain validates (AD flag set, below).

## Verify

```bash
# Records resolve (SVCB type = 64):
dig +short SVCB _index._agents.bastionagentique.com
dig +short SVCB _a2a._agents.bastionagentique.com

# DNSSEC-authenticated answer — look for the `ad` (Authenticated Data) flag:
dig +dnssec _index._agents.bastionagentique.com SVCB | grep -E 'flags:|ad'

# Matches how the scanner checks (DNS-over-HTTPS via Cloudflare):
curl -s 'https://cloudflare-dns.com/dns-query?name=_index._agents.bastionagentique.com&type=SVCB' \
  -H 'accept: application/dns-json'
```

The isitagentready.com scan passes when `checks.discoverability.dnsAid.status` is `"pass"`.
