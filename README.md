<!--
SPDX-FileCopyrightText: 2026 Kevin Monaghan
SPDX-License-Identifier: MIT-0
-->

# FXCM Rust Client

> An unofficial, independent Rust inner client project for FXCM APIs.

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
| Protocol decision | ForexConnect, FIX, or both remain under evaluation |
| Runtime goal | No separately running desktop gateway or proxy; prefer a pure Rust implementation |
| SDK option | An in-process ForexConnect SDK wrapper is under evaluation, not implemented or approved |
| Client layout | One or two clients remains an operator decision |
| Ownership | Provider-native transport, protocol, identities, and wire models |
| Consumer ownership | Credentials, runtime, strategies, routing, risk, accounting, persistence, and reconciliation |
| Implemented API | None; repository setup only |
| License | [MIT No Attribution](LICENSE) (`MIT-0`), matching the other SharurTrading inner clients |

## API and access

FXCM's [API comparison](https://www.fxcm.com/markets/algorithmic-trading/compare-api/)
describes FIX as a standard protocol and ForexConnect as proprietary. Its
[ForexConnect integration guide](https://fxcodebase.com/wiki/index.php/Start_Using_ForexConnect_in_C%2B%2B_project)
documents linking an application against native shared libraries. That documented
SDK path runs inside the application; it does not require a separate Trading Station
process. A Rust wrapper is a potential integration path, but is not a pure Rust
replacement for the provider's protocol implementation.

No public ForexConnect wire specification sufficient for an independent Rust
implementation was identified in the sources reviewed on 2026-10-10. That is an
evidence gap, not proof that such an implementation is impossible.

[Issue #1](https://github.com/SharurTrading/fxcm-rs/issues/1) tracks protocol
feasibility, current SDK/platform support, and access prerequisites before
implementation. FXCM's [API overview](https://www.fxcm.com/markets/algorithmic-trading/api-trading/)
publishes a $5,000 account minimum for FIX; eligibility needs confirmation for the
operator's account. No protocol, capability coverage, or one/two-client architecture
is committed. Older REST documentation is not proof of a current service.

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
