---
name: rust-practice-tutor
description: "Curated Rust teaching resources and concept map for this exercise repository. Use when guiding the student through an exercise, interpreting compiler diagnostics, choosing a progressive hint, reviewing ownership or error handling, or finding the exact Rust Book, standard-library, Rust By Example, or Cargo source to cite. Pairs with the Mentor agent and learning-mode instructions."
user-invocable: false
---

# Rust Practice Tutor

Use this skill to make guidance precise while the student works through the
[curriculum](../../../docs/README.md). Identify the current exercise, choose the smallest
relevant concept below, link one exact source, and ask one question that helps the student
predict the code's behavior. Do not combine several hint rungs in one response.

## Compiler-led workflow

- Read diagnostics from the first error downward; later errors may be consequences.
- Ask the student to identify the expected and actual types before proposing a change.
- For an error code such as `E0382`, use `rustc --explain E0382` and connect its example to
  the student's code.
- Prefer `cargo check --bin <name>` while shaping code and `cargo test --bin <name>` once
  behavior is testable.
- Use `cargo fmt --check` and Clippy after the narrow behavior works; style feedback should
  not obscure the original compiler or logic problem.

Sources:

- Compiler diagnostics: https://doc.rust-lang.org/rustc/diagnostics.html
- Error-code index: https://doc.rust-lang.org/error_codes/error-index.html
- Cargo commands: https://doc.rust-lang.org/cargo/commands/index.html
- Clippy usage: https://doc.rust-lang.org/clippy/usage.html

## Values, functions, and control flow

- **Bindings and mutability.** Bindings are immutable unless marked `mut`; shadowing
  creates a new binding and may change its type. Source:
  https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html
  Guiding question: "Does this value itself need to change, or can the next calculation
  produce a new value?"
- **Scalar types and arithmetic.** Integer division truncates, numeric conversions are
  explicit, and debug/release overflow behavior differs. Source:
  https://doc.rust-lang.org/book/ch03-02-data-types.html
  Guiding question: "Which unit and numeric type does this function promise to return?"
- **Functions and expressions.** A trailing expression is a return value; adding a
  semicolon changes it to `()`. Source:
  https://doc.rust-lang.org/book/ch03-03-how-functions-work.html
- **Branches and loops.** `if` is an expression and its branches must have compatible
  types; `for` is usually the clearest collection loop. Source:
  https://doc.rust-lang.org/book/ch03-05-control-flow.html

## Ownership, borrowing, and text

- **Ownership and moves.** Assignment or a by-value call may move a non-`Copy` value.
  Source: https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
  Guiding question: "Who needs this value after the call, and therefore who should own
  it?"
- **References and mutable borrowing.** References borrow without taking ownership;
  mutation requires an exclusive `&mut` borrow. Source:
  https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
  Guiding question: "During this borrow, is any other code also reading or writing the
  value?"
- **Slices.** `&[T]` and `&str` borrow a view into data and often make better parameters
  than `&Vec<T>` or `&String`. Source:
  https://doc.rust-lang.org/book/ch04-03-slices.html
- **`String` and `&str`.** Owned text is growable; a string slice is a borrowed UTF-8
  view. Sources:
  https://doc.rust-lang.org/book/ch08-02-strings.html and
  https://doc.rust-lang.org/std/string/struct.String.html
  Guiding question: "Must the result outlive the input, or can it remain a borrowed
  view?"
- **Lifetimes.** An annotation describes relationships between references; it does not
  extend how long data lives. Source:
  https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html
  Guiding question: "Which input does the returned reference point into?" Do not suggest
  annotations before that relationship is clear.

## Domain modeling and matching

- **Structs and methods.** Structs group related data; `&self`, `&mut self`, and `self`
  encode how a method uses ownership. Sources:
  https://doc.rust-lang.org/book/ch05-01-defining-structs.html and
  https://doc.rust-lang.org/book/ch05-03-method-syntax.html
  Guiding question: "Should this operation inspect, mutate, or consume the value?"
- **Enums and `match`.** Enums model a closed set of states, and `match` forces exhaustive
  handling. Source: https://doc.rust-lang.org/book/ch06-00-enums.html
  Guiding question: "Can the impossible state be removed by making each valid state an
  enum variant?"
- **`Option<T>`.** Absence is explicit and must be handled rather than represented by a
  sentinel. Source: https://doc.rust-lang.org/std/option/enum.Option.html
- **Patterns.** Destructuring can bind only the fields needed by a branch. Source:
  https://doc.rust-lang.org/book/ch19-00-patterns.html

## Collections, iterators, and closures

- **Vectors.** `Vec<T>` owns a variable-length sequence; slices borrow sequences. Source:
  https://doc.rust-lang.org/book/ch08-01-vectors.html
- **Hash maps and entry API.** A `HashMap<K, V>` owns keys and values; `entry` supports
  update-or-insert behavior. Sources:
  https://doc.rust-lang.org/book/ch08-03-hash-maps.html and
  https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html
  Guiding question: "What should happen on the first occurrence and on every later
  occurrence of a word?"
- **Iterators.** Iterators are lazy until consumed; adapters transform a sequence and
  consumers produce a result. Source:
  https://doc.rust-lang.org/book/ch13-02-iterators.html
  Guiding question: "What is the item type after each adapter?"
- **Closures.** Closures infer parameter types and capture their environment by borrow or
  move as required. Source: https://doc.rust-lang.org/book/ch13-01-closures.html

## Recoverable errors and tests

- **`Result<T, E>` and `?`.** Recoverable failure belongs in the return type; `?` returns
  early after converting a compatible error. Source:
  https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html
  Guiding question: "Should this layer handle the failure, add context, or return it to
  its caller?"
- **Parsing.** `str::parse` uses the target type's `FromStr` implementation. Sources:
  https://doc.rust-lang.org/std/primitive.str.html#method.parse and
  https://doc.rust-lang.org/std/str/trait.FromStr.html
- **Custom error types.** Begin with a small enum whose variants match failures the caller
  can understand. Add `Display` or `Error` only when the exercise needs presentation or
  interoperability.
- **Unit tests.** Use `#[cfg(test)]`, `#[test]`, and assertions near the private code they
  verify. Source: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
  Guiding question: "Which boundary case could make this implementation confidently
  wrong?"

## Command-line and file I/O

- Command-line arguments: https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html
- Reading a file: https://doc.rust-lang.org/book/ch12-02-reading-a-file.html
- `std::env::args`: https://doc.rust-lang.org/std/env/fn.args.html
- `std::fs::read_to_string`: https://doc.rust-lang.org/std/fs/fn.read_to_string.html
- Exit status: https://doc.rust-lang.org/std/process/struct.ExitCode.html

For the mini-project, keep pure text-analysis functions separate from argument and file
I/O. Ask: "Which behavior can be tested with an in-memory `&str` and no process setup?"

## Traits, generics, and conversions (tier 2)

- **Traits and default methods.** A trait names shared behavior; a default method is
  written in terms of the required ones. Source:
  https://doc.rust-lang.org/book/ch10-02-traits.html
  Guiding question: "Which of these methods must every implementor write, and which can be
  derived from the others?"
- **Generics versus trait objects.** A generic bound is monomorphized per concrete type; a
  `dyn Trait` behind a pointer defers the choice to run time and allows mixed collections.
  Sources: https://doc.rust-lang.org/book/ch10-01-syntax.html and
  https://doc.rust-lang.org/book/ch18-02-trait-objects.html
  Guiding question: "Does this collection hold one type or several?"
- **Conversion traits.** `From` is for conversions that cannot fail, `TryFrom` for ones that
  can, and `FromStr` powers `str::parse`. Sources:
  https://doc.rust-lang.org/std/convert/trait.From.html and
  https://doc.rust-lang.org/std/str/trait.FromStr.html
- **`Display` and `Error`.** `Display` writes the sentence a user reads; `Error` makes a
  type usable behind `Box<dyn Error>` and with `?` conversions. Source:
  https://doc.rust-lang.org/std/error/trait.Error.html

## Building iterators and storing closures (tier 2)

- **Implementing `Iterator`.** Write `next` only; every adapter follows from it. Laziness
  means `next` must not consume the whole source. Source:
  https://doc.rust-lang.org/std/iter/trait.Iterator.html
  Guiding question: "How much lookahead does one item need, and where is that state
  stored?"
- **Closure traits.** `FnOnce` may consume captures, `FnMut` may mutate them, and `Fn` only
  reads them; the compiler picks the most permissive one automatically. Source:
  https://doc.rust-lang.org/book/ch13-01-closures.html
- **Returning and storing closures.** `impl Fn` returns one unnamable type;
  `Box<dyn Fn(..) -> ..>` stores many different ones. Source:
  https://doc.rust-lang.org/book/ch20-03-advanced-functions-and-closures.html
- **Choosing a collection.** Ordered iteration suggests `BTreeMap`, membership suggests
  `HashSet`, and both-ended insertion suggests `VecDeque`. Source:
  https://doc.rust-lang.org/std/collections/index.html#when-should-you-use-which-collection

## Smart pointers and concurrency (tier 2)

- **`Box<T>`.** Gives a recursive type a known size and moves data to the heap. Source:
  https://doc.rust-lang.org/book/ch15-01-box.html
  Guiding question: "What size would the compiler have to compute without the indirection?"
- **`Rc<T>`, `RefCell<T>`, and `Weak<T>`.** Shared ownership, run-time borrow checking, and
  non-owning links that break cycles. Sources:
  https://doc.rust-lang.org/book/ch15-04-rc.html,
  https://doc.rust-lang.org/book/ch15-05-interior-mutability.html, and
  https://doc.rust-lang.org/book/ch15-06-reference-cycles.html
  Guiding question: "Which direction of this link should keep the data alive?"
- **Threads, channels, and shared state.** Scoped threads may borrow locals, channels move
  ownership between threads, and `Arc<Mutex<T>>` shares mutable state. Source:
  https://doc.rust-lang.org/book/ch16-00-concurrency.html
  Guiding question: "Is this data moved to one owner, or shared and locked?"
- **Deterministic tests.** Assert on merged results, never on completion order.

## Modules and crate structure (tier 2)

- **Modules and visibility.** Items are private by default; `pub` chooses the public API.
  Source: https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html
- **Library plus binary.** `src/lib.rs` holds testable logic; `src/bin/*.rs` stays thin.
  Source: https://doc.rust-lang.org/cargo/reference/cargo-targets.html
- **Test organization.** Unit tests sit beside private code; integration tests in `tests/`
  see only the public API. Source:
  https://doc.rust-lang.org/book/ch11-03-test-organization.html
  Guiding question: "If the integration test cannot reach this behavior, is the API too
  small or is logic in the wrong place?"

## Exercise map

- **Ex01 Pocket Change:** bindings, integer arithmetic, functions, tuples.
- **Ex02 Temperature Table:** numeric conversions, ranges, loops, formatting, float tests.
- **Ex03 Score Summary:** vectors, slices, `Option`, borrowing, edge cases.
- **Ex04 Tag Cleaner:** `String` versus `&str`, owned output, deduplication.
- **Ex05 Reading List:** structs, enums, methods, `match`, valid-state modeling.
- **Ex06 Inventory Parser:** `Result`, error enums, parsing, `?`, negative tests.
- **Ex07 Word Frequencies:** `HashMap`, entry API, iterators, deterministic sorting.
- **Ex08 Text Inspector:** arguments, file I/O, separation of concerns, end-to-end errors.
- **Ex09 Shape Toolkit:** traits, default methods, generic bounds, trait objects.
- **Ex10 Log Slicer:** lifetime annotations, structs holding references, zero-allocation
  parsing.
- **Ex11 Color Codes:** newtype, `FromStr`, `From`, `TryFrom`, `Display`, `Error`.
- **Ex12 Run Length:** implementing `Iterator`, `Peekable`, laziness, `impl Trait`.
- **Ex13 Event Pipeline:** `Fn`, `FnMut`, `FnOnce`, returning and boxing closures.
- **Ex14 Shift Planner:** `BTreeMap`, `HashSet`, `VecDeque`, derived ordering, grouping.
- **Ex15 Expression Tree:** `Box`, recursive enums, recursion, error propagation.
- **Ex16 Shared Catalog:** `Rc`, `RefCell`, `Weak`, reference counts, cycles.
- **Ex17 Parallel Counter:** scoped threads, channels, `Arc<Mutex<T>>`, determinism.
- **Ex18 Habit Tracker:** modules, library crate, storage trait, integration tests.

Tier 1 (Ex01-Ex08) intentionally avoids concurrency, smart pointers, and trait objects; do
not pull those into a tier-1 hint unless the student asks. Tier 2 (Ex09-Ex18) introduces
them deliberately. Async Rust, macros, unsafe Rust, and external crates stay out of both
tiers.
