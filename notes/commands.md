# Rust Commands

## Basics

1. `rustc <filename>`: Compiles the rust code into a binary executable.
2. `./<filename>`: Runs the binary executable.

## Cargo: The Rust's build system (a.k.a package manager)

1. `cargo --version`: Outputs the version on installed on the machine.
2. `cargo new <project_name>`: Creates a directory by project name containing `Cargo.toml` and a `src` folder with `main.rs`.
   > [!NOTE]
   > `cargo new --vcs=git <project_name / .>`: Initializes or adds git to the specified project with a _gitignore_.
3. `cargo build`: Compiles the code into a binary executable at `./target/debug`.
   > [!NOTE]
   > `cargo build --release`: Compiles the final release version at `./target/release`.
4. `cargo run`: Compiles and executes the code.
5. `cargo check`: Check whether the code compiles without creating an executable.
6. `cargo doc --open`: Opens up docs for reference/help.
