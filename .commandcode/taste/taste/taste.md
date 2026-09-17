# Taste
- Writes in Chinese and expects replies in Chinese (Simplified). Confidence: 0.7
- Wants public-facing GitHub PR/issue replies written in English, even though the surrounding conversation is in Chinese. Confidence: 0.65
- Expects arguments to be evidence-based ("有理有据") — cite concrete sources, upstream code/file line references, and verifiable reasoning rather than assertions. Confidence: 0.6
- When a known upstream bug or version-specific limitation breaks or degrades a feature, wants it documented for users (linking the upstream issue) rather than only handled silently in code — but the user-facing write-up belongs in the tracking issue, not necessarily the README. Confidence: 0.45
- Prefers known problems tracked in a dedicated user-facing "Known issues" GitHub issue (stating symptoms, cause, current status, and both upstream and in-repo follow-up links, removing entries once fixed) so users understand the cause and can follow the resolution — explicitly chose this over adding a README note. Confidence: 0.65
- Wants the user-facing "Known issues" issue pinned to the top of the repo's issue list ("我看有些项目可以把 issue 置顶") so it's the first thing users see, and keeps subsequent known problems consolidated as new entries in that one pinned issue. Confidence: 0.5
- Declines documentation churn he didn't ask for: when the GitHub issue tracker already covers an item, says the README should be left untouched ("不需要修改 README") and expects the working tree to stay otherwise clean. Confidence: 0.6
- Maintains a bilingual README (English `README.md` plus `README.zh-CN.md`) and expects documentation changes applied to both files in sync. Confidence: 0.5
- Prefers important version/compatibility caveats surfaced prominently (at the very top of the README) rather than buried in a later compatibility section. Confidence: 0.5
