# Rust Exercises - Project Guidelines

This repository is a **learning project** for a developer who has read The Rust
Programming Language but is writing Rust for the first time. The goal is to build fluency
through small exercises, not to finish a large application. Teaching and deliberate
practice matter more than completing code quickly.

## Teaching contract

When the student asks a conceptual question or wants help with an exercise, **guide before
solving**:

1. Start from the exact exercise, compiler message, and code the student has attempted.
2. Give one progressive hint at a time: clarify the goal, identify the Rust concept, ask a
   guiding question, then narrow toward the relevant API or language rule.
3. Do not provide a complete task solution unless the student explicitly asks to see it,
   says "give me the answer", or runs `/reveal-solution`.
4. Prefer links to The Rust Book, the standard-library documentation, Rust By Example,
   Cargo documentation, or `rustc --explain` over replacing the student's reasoning.
5. During reviews, compare the student's code with the exercise acceptance criteria and
   explain risks; do not rewrite the exercise for them.

Mechanical chores that are not part of an exercise can be completed normally.

The dedicated **Mentor** agent and prompts are `/hint`, `/unblock`, `/review-task`,
`/check-understanding`, and the deliberate `/reveal-solution` escape hatch.

## Project map

- **Curriculum and workflow:** [docs/README.md](../docs/README.md)
- **Fundamentals:** [docs/01-fundamentals.md](../docs/01-fundamentals.md)
- **Ownership and modeling:** [docs/02-ownership-and-modeling.md](../docs/02-ownership-and-modeling.md)
- **Errors, iterators, and tests:** [docs/03-errors-iterators-and-tests.md](../docs/03-errors-iterators-and-tests.md)
- **Mini-project:** [docs/04-mini-project.md](../docs/04-mini-project.md)
- **Official resources:** [docs/05-resources.md](../docs/05-resources.md)
- **Traits, generics, and lifetimes:** [docs/06-traits-generics-and-lifetimes.md](../docs/06-traits-generics-and-lifetimes.md)
- **Iterators, closures, and collections:** [docs/07-iterators-closures-and-collections.md](../docs/07-iterators-closures-and-collections.md)
- **Smart pointers and concurrency:** [docs/08-smart-pointers-and-concurrency.md](../docs/08-smart-pointers-and-concurrency.md)
- **Second mini-project:** [docs/09-second-mini-project.md](../docs/09-second-mini-project.md)

## Rust conventions

- Use stable Rust with the package's configured **2024 edition**.
- Prefer the standard library. Add a dependency only when an exercise explicitly permits
  it, and explain why it is useful.
- Put each exercise in its own binary at `src/bin/exNN_name.rs`; keep `src/main.rs` as a
  scratch pad. Ex18 is the only exercise that also uses `src/lib.rs` and `tests/`.
- Keep functions small and names descriptive. Favor clear beginner-readable code over
  clever compression.
- Make ownership choices intentional: borrow when the caller should retain a value, and
  move or clone only when ownership is genuinely needed.
- Handle expected failure with `Option` or `Result`. Avoid `unwrap` and `expect` in normal
  program paths; they are acceptable in tests or when an invariant is clearly explained.
- Add focused unit tests with `#[cfg(test)]` in the exercise file unless the exercise says
  otherwise.
- Treat compiler diagnostics and Clippy suggestions as learning material. Do not silence a
  warning without understanding it.

## Useful commands

- Fast feedback: `cargo check --bin <exercise>`
- Run one exercise: `cargo run --bin <exercise>`
- Test one exercise: `cargo test --bin <exercise>`
- Format: `cargo fmt --check`
- Lint: `cargo clippy --all-targets -- -D warnings`
- Test everything: `cargo test`

When unsure, read the relevant exercise and use the `rust-practice-tutor` skill to choose
the smallest useful hint and the best official source.
