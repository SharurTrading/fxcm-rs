<!--
SPDX-FileCopyrightText: 2026 Kevin Monaghan
SPDX-License-Identifier: MIT-0
-->

# Contributing

> Submit complete, narrowly scoped provider-client changes through reviewed PRs.

## Before changing code

1. Read [AGENTS.md](AGENTS.md) and keep the public inner-client boundary.
2. Confirm the current FXCM operation against official, broker-authorized evidence.
3. Separate query, trading mutation, and FIX session replay semantics.
4. Design observable success, refusal, cancellation, and ambiguity outcomes.
5. Land implementation, deterministic tests, rustdoc, and operational docs together.

By intentionally submitting a contribution, you agree to license it under
[MIT-0](LICENSE) and confirm your right to submit it on those terms.

## Local checks

CI pins `cargo-nextest` 0.9.148, `cargo-deny` 0.20.2, Gitleaks 8.30.1, and
`markdownlint-cli2` 0.23.3. Rust is pinned in `rust-toolchain.toml`.

```sh
npx --yes markdownlint-cli2@0.23.3 '**/*.md'
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo nextest run --all-features --locked --no-tests pass
cargo test --doc --all-features --locked
RUSTDOCFLAGS='-D warnings' cargo doc --all-features --no-deps --locked
cargo deny --locked check
cargo package --locked
gitleaks git --no-banner --redact --log-opts='--all' .
```

The scaffold has no behavior tests. Remove `--no-tests pass` when real behavior
tests are added. A passing empty suite is not an API or readiness claim.

## Review and tracking

| Requirement | Outcome |
| --- | --- |
| Classification | Matching kind label and the organization's Priority issue field on the PR; issues also have native type and difficulty |
| Agent attribution | Exact authoring model disclosed in every agent-authored GitHub body/comment |
| Review | Operator starts the review agent manually; findings cite stable rules |
| Deferred work | Completed in the change or linked to a classified GitHub issue |
| Landing | Required `quality` check passes and review conversations are resolved |

## Security and provider material

Use synthetic identifiers and messages. Do not commit credentials, real account
data, FIX session logs, network captures, or restricted FXCM documentation. Report
vulnerabilities using [SECURITY.md](SECURITY.md).

Normal tests run offline without credentials. Live probes must be read-only,
ignored, and deliberately invoked; trading campaigns require separate operator
authorization. No GitHub or Cargo release is part of a routine contribution.
