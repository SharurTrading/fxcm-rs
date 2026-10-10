<!--
SPDX-FileCopyrightText: 2026 Kevin Monaghan
SPDX-License-Identifier: MIT-0
-->

# FXCM Rust Client

> An unofficial, independent Rust inner client project for FXCM FIX 4.4.

[![CI](https://github.com/SharurTrading/fxcm-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/SharurTrading/fxcm-rs/actions/workflows/ci.yml)

Maintained by SharurTrading. This project is not affiliated with or endorsed by FXCM.

> [!WARNING]
> **WIP — unreleased and untested for live trading.** This repository currently
> contains the crate scaffold, contributor rules, and CI only. No connection,
> market-data, account, or trading API is implemented. A green CI badge verifies
> repository checks; it does not establish live trading readiness. Cargo registry
> publishing is disabled with `publish = false`.

## At a glance

| Area | Contract |
| --- | --- |
| Crate | `fxcm-client`, Rust 2024, pinned stable toolchain |
| Intended protocol | FXCM FIX 4.4, connected directly from Rust |
| Runtime boundary | No desktop gateway, Java runtime, ForexConnect native SDK, FFI bridge, or separate proxy |
| Ownership | Provider-native transport, protocol, identities, and wire models |
| Consumer ownership | Credentials, runtime, strategies, routing, risk, accounting, persistence, and reconciliation |
| Implemented API | None; repository setup only |
| License | [MIT No Attribution](LICENSE) (`MIT-0`), matching the other SharurTrading inner clients |

## API and access

FXCM's [API overview](https://www.fxcm.com/markets/algorithmic-trading/api-trading/)
describes FIX as a direct trading and live-price interface and publishes a $5,000
Trading Station account minimum. Its [API comparison](https://www.fxcm.com/markets/algorithmic-trading/compare-api/)
identifies FIX as a standard socket protocol. These public sources were checked on
2026-10-10; account eligibility, credentials, transport details, and the current
message specification require broker confirmation.

[Issue #1](https://github.com/SharurTrading/fxcm-rs/issues/1) tracks that contract
and access work before implementation. Historical data and the complete account
surface are not claimed. Older REST/Socket.IO documentation is not proof of a
currently provisioned service.

## Repository map

| Path | Purpose |
| --- | --- |
| [`src/lib.rs`](src/lib.rs) | Documentation-only crate root; no public operations |
| [`AGENTS.md`](AGENTS.md) | Stable rules for coding and review agents |
| [`CONTRIBUTING.md`](CONTRIBUTING.md) | Local gates and contribution process |
| [`SECURITY.md`](SECURITY.md) | Private reporting and secret-handling boundary |
| [CI workflow](.github/workflows/ci.yml) | Markdown, Rust, supply-chain, package, and secret checks |

## Development

Read [CONTRIBUTING.md](CONTRIBUTING.md) and [AGENTS.md](AGENTS.md) before changing
the crate. CI runs on GitHub-hosted Linux runners without broker credentials.
Agent reviews are started manually by the operator; merges require the `quality`
check and resolved review conversations.

## License

Repository-authored material is licensed under [MIT-0](LICENSE). Provider documents
and SDKs retain their own terms; restricted material must not be committed here.
