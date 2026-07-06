# dnsglobe

[![crates.io](https://img.shields.io/crates/v/dnsglobe)](https://crates.io/crates/dnsglobe)
[![license](https://img.shields.io/badge/license-MIT-blue)](LICENSE)

**A global DNS propagation checker for your terminal** — a Rust TUI that
queries 39 public DNS resolvers around the world in parallel, compares their
answers, and shows the propagation of your record on a world map.

```text
$ dnsglobe --once example.com
example.com A

Google Public DNS      Anycast  8.8.8.8          OK         13ms  ttl=243     104.20.23.154, 172.66.147.243
Cloudflare             Anycast  1.1.1.1          OK         11ms  ttl=163     104.20.23.154, 172.66.147.243
Quad9                  CH/Any   9.9.9.9          OK         11ms  ttl=215     104.20.23.154, 172.66.147.243
Level3                 US       4.2.2.2          OK         21ms  ttl=223     104.20.23.154, 172.66.147.243
CIRA Canadian Shield   CA       149.112.121.10   OK         91ms  ttl=188     104.20.23.154, 172.66.147.243
DNS4EU                 EU/Any   86.54.11.100     OK         11ms  ttl=127     104.20.23.154, 172.66.147.243
AdGuard DNS            EU/Any   94.140.14.14     OK         15ms  ttl=242     104.20.23.154, 172.66.147.243
Yandex DNS             RU       77.88.8.8        DIFFERS    48ms  ttl=206     8.47.69.0, 8.6.112.0
Comss.one              RU       83.220.169.155   DIFFERS    49ms  ttl=5408    8.47.69.0, 8.6.112.0
Shecan                 IR       178.22.122.100   OK         96ms  ttl=85      104.20.23.154, 172.66.147.243
114DNS                 CN       114.114.114.114  OK        146ms  ttl=177     104.20.23.154, 172.66.147.243
ByteDance Public DNS   CN       180.184.1.1      OK        301ms  ttl=599     104.20.23.154, 172.66.147.243
Telstra                AU       139.130.4.4      OK        294ms  ttl=242     104.20.23.154, 172.66.147.243
UOL                    BR       200.221.11.100   OK        196ms  ttl=230     104.20.23.154, 172.66.147.243
...                    (39 resolvers total)

39 of 39 responding · 0 unreachable · 2 answer group(s)
propagation (37/39 responding): 104.20.23.154, 172.66.147.243
```

Think dnschecker.org / whatsmydns.net, but in your terminal, with watch mode:
start a check and it re-polls until the record has propagated everywhere.

Resolvers span the global anycast networks (Google, Cloudflare, Quad9),
North America, Europe, Russia, the Middle East, East Asia, and the southern
hemisphere (Telstra AU, SafeSurfer NZ, UOL BR) — each queried directly, so
you see every server's own current view of the record.

Each resolver is queried directly (no cache, EDNS0, TCP fallback for
truncated answers), so what you see is each server's own current view of the
record. Answers sharing any record are grouped together — so round-robin DNS
(each resolver caching a different subset of an IP pool) counts as one
consistent answer, not twenty conflicting ones. The propagation gauge shows
how many resolvers are in the majority group; outliers are flagged
`≠ DIFFERS` once all results are in.

On terminals ≥150 columns wide, a world map appears on the right with one
dot per resolver, colored by status (green agrees, magenta differs, red
error, yellow in flight).

## Usage

Install:

```sh
brew install besmirzanaj/tap/dnsglobe   # Homebrew (macOS/Linux)
cargo install dnsglobe               # from crates.io
# or grab a prebuilt binary from the GitHub Releases page
```

Run:

```sh
dnsglobe                            # start empty, type a domain
dnsglobe example.com                # query immediately and watch
dnsglobe --once example.com TXT    # no TUI: print results, exit (for scripts)
dnsglobe --once example.com --output json   # machine-readable JSON
dnsglobe --once example.com A --output csv  # CSV (RFC 4180), values joined with |
```

### Keys

| Key            | Action                          |
| -------------- | ------------------------------- |
| type / ⌫ / Del | edit domain                     |
| ←/→ / Home/End | move cursor in the domain field |
| Enter          | start the check and watch: re-polls every 30 s until propagation reaches 100% |
| Ctrl+R         | stop or resume watching         |
| Tab / Shift-Tab | select record type (A, AAAA, CNAME, MX, NS, TXT, SOA) |
| ↑/↓ / PgUp/PgDn | scroll the resolver table |
| Ctrl+S         | cycle table sort: resolver / location / time / status / answer |
| Ctrl+U         | clear domain                    |
| Esc / Ctrl+C   | quit                            |

## Notes

- Several resolvers are anycast networks, so the responding node is the one
  nearest to you; the location column is the operator's home region.
- Resolver list lives in `src/resolvers.rs` — add or remove entries freely.
  Every entry was verified to answer external queries; many well-known ISP
  resolvers (and, notably, all major African ones) refuse queries from
  outside their network, so they can't be included.
