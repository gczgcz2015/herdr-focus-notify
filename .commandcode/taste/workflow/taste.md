# Workflow

- When asking for a PR review, expects the reviewer to consult the referenced upstream issues/PRs and verify claims against the upstream source instead of accepting the PR's stated diagnosis at face value. Confidence: 0.55
- When pointing at a reported issue, expects the claim to be verified against the actual source code and reproduced/checked locally (with concrete log evidence, timestamps, env facts) before accepting it as a real bug or proposing a fix. Confidence: 0.5
- Probes the causal mechanism behind each claim ("why would that happen?") and pushes back when a conclusion is inferred from absence of evidence — expects the explanation to be backed by measured data and overreaching inferences to be retracted plainly. Confidence: 0.5
