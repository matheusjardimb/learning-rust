# learning-rust

Small projects used for learning Rust, based on content presented at https://www.udemy.com/course/rust-fundamentals/

# Reference material

- https://www.rust-lang.org/tools/install
- https://crates.io/

# Snippets

```bash
# Creates a project named 'example'
cargo new example

# Installs dependency at OS level
cargo install cargo-expand

# TBD
cargo expand
# List all toolchains available
rustup toolchain list
# Sets up a new toolchain (replace 'NAME')
rustup toolchain NAME


# Builds and run the project
cargo build
cargo run

```

# Tips

- Use snake case for vars and functions
- Last line in a function without a ';' by the end will behave as a 'return'
- methods with a '!' at the end are 'macros': used for meta-programming, method can be called with variable number of
  params
