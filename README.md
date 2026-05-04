# 🦀 Rust Journey

> Learning Rust — a systems programming language focused on safety, speed, and concurrency.
> This repo tracks my progress, notes, and projects as I go from zero to Rustacean.

## Progress Tracker

| Chapters                   | Status      | Started    | Completed  |
| -------------------------- | ----------- | ---------- | ---------- |
| Introduction               | Completed   | 30/04/2026 | 30/04/2026 |
| Programmig a Guessing Game | Completed   | 30/04/2026 | 30/04/2026 |
| Core Fundamentals          | Completed   | 30/04/2026 | 04/05/2026 |
| Ownership System           | In progress | 04/05/2026 | -          |

## Concepts Covered

### Core Fundamentals

- [x] Variables, mutability & shadowing
- [x] Data types (scalar & compound)
- [x] Functions & control flow
- [x] Loops (`loop`, `while`, `for`)
- [x] Comments & documentation (`//`, `///`)
- [x] Constants & statics (`const`, `static`)

### Ownership System _(Rust's killer feature)_

- [ ] Ownership rules
- [ ] Move semantics
- [ ] Borrowing & references
- [ ] Mutable vs immutable references
- [ ] Dangling references
- [ ] Slices (`&str`, `&[T]`)

### Structs & Enums

- [ ] Defining structs & methods
- [ ] Tuple structs & unit structs
- [ ] Enums & `Option<T>`
- [ ] Pattern matching with `match`
- [ ] `if let` & `while let`
- [ ] Destructuring

### Error Handling

- [ ] `Result<T, E>` and `Option<T>`
- [ ] `?` operator
- [ ] `unwrap`, `expect`, `map`, `and_then`
- [ ] Panic vs recoverable errors
- [ ] Custom error types

### Generics, Traits & Lifetimes

- [ ] Generic types & functions
- [ ] Defining and implementing traits
- [ ] Default trait implementations
- [ ] Trait bounds & `where` clauses
- [ ] Lifetime annotations
- [ ] Lifetime elision rules
- [ ] Common traits (`Display`, `Debug`, `Clone`, `Copy`, `From`, `Into`)

### Collections & Iterators

- [ ] `Vec<T>`
- [ ] `HashMap<K, V>` & `HashSet<T>`
- [ ] `String` vs `&str`
- [ ] Closures & capturing environment
- [ ] Iterator trait & lazy evaluation
- [ ] Iterator adapters (`map`, `filter`, `collect`, `fold`, `enumerate`, `zip`)
- [ ] `impl Trait` & `dyn Trait`

### Modules & Packages

- [ ] Modules (`mod`) & visibility (`pub`)
- [ ] `use` & `as` for imports
- [ ] Crates & `Cargo.toml`
- [ ] Workspaces
- [ ] External crates from crates.io

### Testing

- [ ] Unit tests (`#[test]`)
- [ ] Integration tests
- [ ] `assert!`, `assert_eq!`, `assert_ne!`
- [ ] Test-driven development in Rust
- [ ] `cargo test`

### Smart Pointers

- [ ] `Box<T>` — heap allocation
- [ ] `Rc<T>` — reference counting
- [ ] `RefCell<T>` — interior mutability
- [ ] `Arc<T>` — atomic reference counting
- [ ] `Mutex<T>` & `RwLock<T>`
- [ ] `Deref` & `Drop` traits

### Concurrency

- [ ] Threads (`std::thread`)
- [ ] Message passing with channels (`mpsc`)
- [ ] Shared state concurrency
- [ ] `Send` & `Sync` traits
- [ ] `async` / `await` basics
- [ ] Tokio runtime
- [ ] `Future` trait

### Advanced

- [ ] Closures as function parameters & return types
- [ ] Function pointers
- [ ] Macros (`macro_rules!`)
- [ ] Procedural macros
- [ ] Unsafe Rust & raw pointers
- [ ] FFI (Foreign Function Interface)
- [ ] `std::mem` & low-level memory control
- [ ] Zero-cost abstractions

## Projects

| #   | Project              | Concepts Practiced    | Status |
| --- | -------------------- | --------------------- | ------ |
| 1   | Number Guessing Game | Variables, I/O, loops | Done   |

## Notes

> Organized by topic — each linked to its own file in `/notes`.

- [Core Fundamentals](./notes/core_fundamentals.md) — Variables, Mutability, Datatypes, Control Flow and Loops.

## Key Takeaways

> Things that clicked, surprised me, or tripped me up.

- _Concept of Shadowing where the same variable is defined with the keyword `let`._
- _Rust is an expression-based language it has something called Statements(Returns no value) and Expressions(Returns value)._

## Resources

| Type     | Resource                                                                    |
| -------- | --------------------------------------------------------------------------- |
| Book     | [The Rust Programming Language (The Book)](https://doc.rust-lang.org/book/) |
| Book     | [Rust by Example](https://doc.rust-lang.org/rust-by-example/)               |
| YouTube  | [Let's Get Rusty](https://www.youtube.com/@letsgetrusty)                    |
| Practice | [Exercism – Rust Track](https://exercism.org/tracks/rust)                   |

## Setup

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify
rustc --version
cargo --version

# Run a project
cargo new my-project
cd my-project
cargo run
```

_Started: 04/05/2026 · Updated regularly as I progress 🚀_
