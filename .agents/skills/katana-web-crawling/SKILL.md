---
name: katana-web-crawling
description: Guides use of ProjectDiscovery Katana for web crawling and spidering in security testing and recon workflows. Covers installation, standard vs headless mode, scope and rate limits, JSONL output, and piping from httpx or URL lists. Use when the user mentions Katana, projectdiscovery/katana, web crawling, spidering, endpoint discovery, attack surface mapping, or chaining crawlers in automation pipelines.
---

# Katana web crawling

**Katana** is a fast crawler/spider from [ProjectDiscovery](https://github.com/projectdiscovery/katana), aimed at **automation pipelines** (URLs in → discovered endpoints out). Official docs and flags: repository **README** and `katana -h`.

## Scope and ethics

Use **only** on systems you **own** or are **explicitly authorized** to test (contract, bug bounty program rules, internal env). **Crawl gently:** set **concurrency**, **rate limits**, and **depth** to reduce load. Misuse can violate law and terms of service-**you are responsible** for your actions (tool ships with that warning).

## Installation

**Go (requires Go 1.25+ per upstream; verify current README if install fails):**

```bash
CGO_ENABLED=1 go install github.com/projectdiscovery/katana/cmd/katana@latest
```

**Docker:**

```bash
docker pull projectdiscovery/katana:latest
docker run projectdiscovery/katana:latest -u https://example.com
```

Headless in Docker often needs `-system-chrome` and Chrome/Chromium available-see upstream **Docker** section.

## Input

- Single/multiple URLs: `-u https://a.com` or comma-separated URLs
- File: `-list urls.txt`
- **STDIN:** `echo https://example.com | katana` or `cat domains | httpx | katana`

## Modes

| Mode | When |
|------|------|
| **Standard (default)** | Fast; uses Go HTTP client; **no** full JS/DOM render-may miss post-render routes |
| **Headless** (`-headless`) | Browser context; better for **JS-heavy** apps; optional `-system-chrome` |

Enable **JS file parsing** for more endpoints: `-js-crawl` (`-jc`). `-jsluice` is heavier.

## Flags to know first

| Flag | Purpose |
|------|---------|
| `-d`, `-depth` | Max crawl depth (default **3**) |
| `-c`, `-concurrency` | Parallel fetchers |
| `-rl`, `-rate-limit` | Max requests per second |
| `-ct`, `-crawl-duration` | Cap total crawl time (e.g. `5m`) |
| `-cs` / `-cos` | **In-scope** / **out-of-scope** URL regex |
| `-ns` | Disable default host scope if you need cross-host (use carefully) |
| `-iqp` | Ignore same path with different query strings |
| `-fs`, `-filter-similar` | Reduce near-duplicate paths |
| `-kf`, `-known-files` | `robots.txt` / `sitemap.xml` etc. (min depth **3** for full coverage per docs) |
| `-j`, `-jsonl` | **JSONL** output for scripting |
| `-o`, `-output` | Write to file |
| `-sr`, `-store-response` | Store HTTP for review (disk use) |
| `-proxy` | HTTP/SOCKS5 proxy |
| `-H` | Extra headers (auth, cookies) via `header:value` |

Run `katana -h` for the full list (filters, form fill, tech detect, TLS options, etc.).

## Minimal examples

```bash
katana -u https://example.com -d 2 -silent
```

```bash
katana -u https://example.com -jsonl -o endpoints.jsonl
```

```bash
katana -list seeds.txt -d 3 -cs '.*\.example\.com.*' -rl 30 -jsonl
```

**Headless (JS-heavy target):**

```bash
katana -u https://example.com -headless -d 2
```

## Pipelines

Common pattern: resolve live HTTP first, then crawl:

```bash
cat domains.txt | httpx -silent | katana -jsonl -o crawl.jsonl
```

Combine with other PD tools (naabu, nuclei, etc.) only in **authorized** assessments.

## Troubleshooting

- `CGO_ENABLED=1` required for **go install** per README.
- **Headless** failures: try `-system-chrome`, ensure Chrome/Chromium installed, or use Docker image with documented Chrome setup.
- **Health check:** `-health-check` / `-hc`.

## References

- Source and releases: [github.com/projectdiscovery/katana](https://github.com/projectdiscovery/katana)
