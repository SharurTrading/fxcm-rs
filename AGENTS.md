<!--
SPDX-FileCopyrightText: 2026 Kevin Monaghan
SPDX-License-Identifier: MIT-0
-->

# FXCM Rust Client Agent Guide

> Repository rules for implementation and review of `fxcm-client`.

This is a public, standalone MIT-0 inner client. Rules describe the required
implementation contract; they are not claims that the scaffold implements it.
No API operations currently exist. Keep work narrowly scoped and operator driven.

## Boundary and provider contract

| ID | Rule |
| --- | --- |
| **FX-BOUNDARY-01** | Own FXCM transport, protocol lifecycle, provider identities, validation, and wire models. No consuming platform dependency, canonical instrument translation, strategy, GUI, RMS, portfolio accounting, or storage policy. |
| **FX-DIRECT-01** | Use the selected FXCM FIX 4.4 interface directly from Rust. No required Trading Station process, desktop gateway, Java runtime, ForexConnect native SDK, FFI bridge, or separate proxy. Client layout remains an operator decision. Ordinary Rust dependencies require review. |
| **FX-CONTRACT-01** | Implement only a current broker-authorized FXCM FIX contract. Cite dated official evidence in PRs. Verify message extensions, endpoints, encryption, access, and limits before implementing them. Never infer a wire specification from SDK method documentation or guess capabilities from another venue's FIX dialect or older REST documentation. |
| **FX-COVERAGE-01** | Public capability claims require typed operations, typed success/refusal outcomes, and deterministic tests. Unsupported operations are declared absent only with provider evidence. CI success is not live trading readiness. |
| **FX-LICENSE-01** | Authored material is MIT-0 with SPDX headers in each comment-capable file. Generated files and formats without comments are exempt. Third-party material retains its own terms. Never copy proprietary platform code or restricted provider documents into this repository. Registry publishing and releases require a separately reviewed decision. |

## Safety and lifecycle

| ID | Rule |
| --- | --- |
| **FX-DECIMAL-01** | Financial prices, quantities, balances, fees, margin, and PnL use exact `rust_decimal::Decimal` parsing/serialization without a float round-trip. Documented integral protocol counts retain their exact integer representation. Checked arithmetic returns a typed failure. Signed prices are not subject to a blanket positivity invariant. |
| **FX-IDENTITY-01** | Provider identities use distinct validated newtypes. A trading request names its required account, instrument, and caller order identity explicitly. Do not infer identity from display text or fabricate missing state. |
| **FX-SECRET-01** | Callers supply credentials. Do not read environment/config, persist secrets, expose secret accessors, or include secrets in logs, errors, fixtures, `Debug`, or displayed URLs. Secret owners redact formatting and zeroize on drop. |
| **FX-TRANSPORT-01** | Remote transport requires authenticated encryption and verified peer identity. Plaintext is allowed only for exact loopback fixtures. The provider's provisioned secure transport must be confirmed; do not silently downgrade or assume TLS details. Bound frames, decoded messages, bodies, queues, pending work, and retained events. Overflow is an explicit failure/gap. |
| **FX-RUNTIME-01** | The caller owns the asynchronous runtime. No hidden runtime, blocking I/O on executor threads, lock held across I/O, or untracked background tasks. Tasks have explicit cancellation and teardown. |
| **FX-SESSION-01** | Session state and sequence authority have one writer. Logon, heartbeat, logout, resend, duplicate handling, sequence reset, and reconnect follow the FXCM contract. Revision/generation fences reject stale completion. Never treat transport readiness as authenticated session readiness. |
| **FX-MUTATION-01** | Trading mutations are single-attempt at the application level. Timeout, cancellation, or disconnect after possible transmission is an explicit ambiguous outcome; it never triggers a new order automatically. FIX session replay follows documented sequence/duplicate semantics and cannot become an application retry. Fence affected-account mutations across clones until the caller acknowledges authoritative reconciliation. |
| **FX-RATE-01** | Every outbound attempt has admission and a caller deadline. Budgets and provider cooldown are shared across clones. A mutation without immediate capacity fails locally before transmission. Never invent undocumented provider limits. |
| **FX-RECOVERY-01** | The client reports disconnects, gaps, uncertain execution, and resynchronization needs. Consumers own authoritative subscriptions and account reconciliation. Do not silently drop accepted evidence or claim account completeness before provider snapshot/recovery finishes. |
| **FX-INVARIANT-01** | Representable invalid states return focused `thiserror` variants, preserving accepted evidence and avoiding partial business-state mutation. Missing required state never becomes a guessed value, benign absence, or success. Test the owning caller's observable failure and state preservation. |
| **FX-DETERMINISM-01** | Inject clocks and randomness used by protocol behavior. Protocol timestamps are UTC instants. Ambient wall time and randomness belong only in explicitly justified construction boundaries. |

## Rust and documentation

| ID | Rule |
| --- | --- |
| **FX-RUST-01** | Unsafe Rust is forbidden. Introducing unsafe code or weakening that lint requires explicit operator approval covering both the code and its lint exception. Production paths never use unwrap, expect, panic, todo, unimplemented, unreachable, or assertions to handle failures. Tests may use assertions. Lints live in Cargo/clippy configuration; per-item exceptions require a short justification. |
| **FX-STYLE-01** | Production paths use `crate::`, functions have one blank line between them, and every public item has rustdoc. Fallible public APIs document `# Errors`. Source comments explain the local contract without rule IDs, issue/PR numbers, history, or plans. |
| **FX-SIZE-01** | Handwritten production files target 400 lines and remain below 600; crate-root/module wiring targets 200. Split by protocol ownership or provider capability. Add a crate only for a directed boundary, not a size limit. |
| **FX-DOC-01** | Update documentation with code. READMEs open with a title and one-line summary, use tables for parallel facts, numbered steps for procedures, fenced runnable commands, and GitHub callouts for readiness limits. Rationale lives beside its module. Markdown must lint clean. Preserve the root WIP warning until the operator approves readiness. |
| **FX-LOG-01** | Use `tracing` for structured logs, with sanitized provider/caller identities. Never log raw FIX frames, authorization fields, or account payloads. Order audit events retain outcome evidence without revealing secrets. |

## Tests and CI

| ID | Rule |
| --- | --- |
| **FX-TEST-01** | Every test protects a named observable behavior and has been seen to fail without that behavior. Expected values come from independent protocol examples or hand calculations. Do not add construction-only, compilation-only, or removed-feature regression tests. |
| **FX-FAKES-01** | The implementation runs for real; fakes replace network, time, randomness, or other genuine boundaries. Tests use deterministic synthetic data, controlled time, no sleeps, no shared mutable state, and no ordering dependency. |
| **FX-TEST-LOCATION-01** | Unit tests live in a final `#[cfg(test)] mod tests` in the owning file. Public integration tests live in `tests/<behavior>.rs`; shared support lives in `tests/support/mod.rs`. Executable doc examples assert their outcomes. |
| **FX-LIVE-01** | Normal CI is credential-free. Live tests require explicit operator invocation and credentials, are ignored with a precise reason, and are read-only. Any trading acceptance campaign requires separate operator authorization and a broker-approved practice account. Never commit live captures or identifiers. |
| **FX-CI-01** | A single `quality` job runs Markdown lint, fmt, strict all-target/all-feature Clippy, nextest, doctests, rustdoc, dependency/license checks, package verification, and secret scan on GitHub-hosted runners. Cargo resolution uses `--locked`. Pin actions to full commit SHAs. No custom policy scanners. |
| **FX-EMPTY-01** | The documentation-only scaffold permits nextest's `--no-tests pass`. Once real behavior tests are introduced, remove that allowance. Do not manufacture tests just to make an empty crate appear implemented. |
| **FX-SUPPLY-01** | Keep the pinned stable toolchain, MSRV, lockfile, and reviewed dependencies current. Use only necessary dependency features. Never suppress an active security advisory. Review transport defaults, secret logging, cancellation, retries, and runtime ownership when selecting dependencies. |

## Process

| ID | Rule |
| --- | --- |
| **PROC-REVIEW** | Work lands by PR into `main`. The operator manually starts the configured review agent. Review findings cite this guide's IDs. Every inline/outdated thread and outside-diff recommendation gets an explicit disposition; adopted findings are fixed, declined findings receive a reason, and conversations are resolved before merge. The `quality` gate is required. Operator review covers trading/reconciliation semantics, secrets, precision, and new dependencies. |
| **PROC-SCOPE** | One complete, narrow feature per PR. Keep implementation, meaningful tests, and documentation together. Broader implementation or architecture remains operator driven. |
| **PROC-FOLLOWUP** | Every named follow-up is completed in the same change or opened as a GitHub issue before merge, with its number cited where mentioned. Operator GUI/live acceptance checks stay in the PR checklist; discovered defects still need a fix or issue. |
| **PROC-ISSUE-TRIAGE** | Classify each issue with exactly one native type (`Bug`, `Feature`, `Task`), its matching kind label (`bug`, `enhancement`, `task`), the organization's `Priority` issue field, and exactly one difficulty label (`difficulty: hard`, `difficulty: medium`, `difficulty: easy`). Correct classification when touching an issue. |
| **PROC-PR-TRIAGE** | A PR carries one matching kind label and inherits its linked issue's Priority issue-field value. A PR with no linked issue states why and sets its own Priority using the same ladder. Priority is never duplicated as a label or prose classification. |
| **PROC-ATTRIB** | Every agent-authored issue/PR body, comment, review, inline comment, or release posted under the operator's login states the exact model in that artifact's visible body, for example `AI-Agent: <actual model identifier>`. Never invent an identifier or attribute human-authored posts. |
| **PROC-DEP-DECIDE** | Dependency defects are investigated and tracked inside SharurTrading first. Do not publish third-party reports or comments without explicit operator instruction. |

Priority uses `Urgent` for current money/order/account errors or a blocked live
session, `High` for the next live session or active-plan step, `Medium` for ordinary
work by default, and `Low` for deferrable polish. Difficulty uses `hard` for protocol
research, architecture, money-path, concurrency, and lifecycle decisions; `medium`
for bounded engineering; and `easy` for mechanical work or operator-only checks.
