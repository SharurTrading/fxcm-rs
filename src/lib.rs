// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Repository scaffold for an unofficial provider-native FXCM client.
//!
//! No network, authentication, market-data, account, or trading operations are
//! implemented. Protocol and client layout remain undecided. The integration
//! targets an in-process client without a separately running gateway; a pure Rust
//! implementation and an SDK-backed `ForexConnect` wrapper are distinct options.
//! Consumers own their runtime, credentials, policy, and account reconciliation.
