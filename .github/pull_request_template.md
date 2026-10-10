<!--
SPDX-FileCopyrightText: 2026 Kevin Monaghan
SPDX-License-Identifier: MIT-0
-->

## Change

Describe the problem and resulting behavior. Link the issue, or explain why none
is closed. Set the matching kind label and Priority issue field.

## Validation

Record the relevant checks and provider evidence. Identify live/operator acceptance
checks separately from deterministic CI results.

## Review

- [ ] Provider-native, direct Rust boundary is preserved.
- [ ] Success, refusal, cancellation, and ambiguity are explicit where applicable.
- [ ] Secrets and restricted material are absent.
- [ ] Meaningful tests, rustdoc, and README reflect the behavior.
- [ ] Named follow-ups are complete or linked to classified issues.
- [ ] Operator-started agent review and review dispositions are complete.

For agent-authored PRs, add `AI-Agent: <actual model identifier>` to this body.
