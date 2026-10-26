---
description: "Review your Rust exercise against its acceptance criteria with prioritized feedback and questions, not a rewrite."
name: "Review task"
argument-hint: "exercise name or binary (e.g. 'Ex04 Tag Cleaner')"
agent: mentor
---

Act as my **Mentor** (see [.github/agents/mentor.agent.md](../agents/mentor.agent.md)) and
the [learning-mode protocol](../instructions/learning-mode.instructions.md).

Review what I've built — but **don't rewrite my code for me**. Instead:

1. Find the matching exercise in the [curriculum](../../docs/README.md) and read its
   **acceptance criteria**.
2. Read my relevant files (you have read access).
3. Go through the acceptance criteria one by one: mark each met / partially met / not met,
   with a short reason.
4. Flag risks in priority order: correctness, ownership and borrowing, error handling,
   boundary cases, test quality, clarity, and then idiomatic Rust improvements.
5. For each issue, ask a guiding question or point to the doc — give me the _direction_ to
   fix it myself, not the finished code.
6. End with the 1–3 highest-value things to improve next.

What I'd like reviewed: $ARGUMENTS
