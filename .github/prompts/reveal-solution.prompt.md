---
description: "Deliberate escape hatch: ask the Mentor for the FULL solution to a task, with a complete explanation. Use only when you genuinely want the answer."
name: "Reveal solution"
argument-hint: "exactly what you want the full solution for"
agent: mentor
---

Act as my **Mentor** (see [.github/agents/mentor.agent.md](../agents/mentor.agent.md)).

This is my **explicit request** to see the full solution — the hints-only rule is lifted
for this message. Please:

1. Give me the **complete, idiomatic solution** for what I ask below (working code that
   fits this project's constraints: stable Rust 2024, standard library unless explicitly
   permitted otherwise, focused functions, and unit tests).
2. **Explain why it works**, especially the ownership, borrowing, types, control flow,
   and error propagation that matter; note reasonable alternatives or trade-offs.
3. Name the concept it teaches and link the relevant Rust Book, standard-library, Rust By
   Example, or Cargo documentation.
4. Give me one **small variation to try myself** afterwards, so it sticks.

Be honest if there's more than one reasonable approach, and say which you'd choose and why.

What I want the full solution for: $ARGUMENTS
