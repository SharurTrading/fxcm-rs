// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Repository scaffold for an unofficial provider-native FXCM FIX 4.4 client.
//!
//! No network, authentication, market-data, account, or trading operations are
//! implemented. The intended client connects directly from Rust, without a
//! separately running gateway or the `ForexConnect` native SDK. Consumers own
//! their runtime, credentials, trading policy, and account reconciliation.
