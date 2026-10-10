// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Repository scaffold for an unofficial provider-native FXCM FIX 4.4 client.
//!
//! No network, authentication, market-data, account, or trading operations are
//! implemented. The client targets direct Rust FIX connections without a desktop
//! gateway, Java runtime, native provider SDK, or separate proxy. Client layout
//! remains undecided. Consumers own their runtime, credentials, trading policy,
//! derived account values, and reconciliation.
