<!--
SPDX-FileCopyrightText: 2026 Kevin Monaghan
SPDX-License-Identifier: MIT-0
-->

# Security

> Report vulnerabilities privately and keep broker credentials out of public artifacts.

Use [GitHub private vulnerability reporting](https://github.com/SharurTrading/fxcm-rs/security/advisories/new).
Do not put credentials, real account identifiers, raw FIX frames, or exploit details
in ordinary issues or PRs. If a credential is exposed, revoke or rotate it through
the broker and retain only minimal redacted evidence.

## At a glance

| Boundary | Requirement |
| --- | --- |
| Credentials | Supplied by callers; never discovered, persisted, logged, or exposed by the library |
| Transport | Broker-authorized authenticated encryption with verified peer identity |
| Remote input | Bounded framing, decoding, queues, and retained events |
| Trading uncertainty | Explicit ambiguous outcomes and caller-owned reconciliation; no automatic new-order retry |
| Dependency advisories | No suppression of an active advisory |
| Fixtures | Synthetic, deterministic, and credential-free |

The crate currently has no transport or trading implementation. These requirements
govern code added through reviewed changes. Unsupported transports must fail
explicitly. Any native SDK/FFI integration requires an explicit operator decision
and review of its credential, threading, lifetime, packaging, and unsafe-code boundary.
