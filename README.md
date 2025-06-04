# MiniGrep

A simple command-line grep utility written in Rust, following the tutorial from "The Rust Programming Language" book.

## Features

- Search for specific text patterns in files
- Command-line argument parsing with error handling
- File reading and content display

## Usage

```bash
cargo run <search_term> <file_path>
```

### Example

```bash
cargo run "hello" story.txt
```

## Build

```bash
cargo build
```

## Progress

### ✅ Completed
- [x] Basic command-line argument parsing
- [x] Config struct for managing arguments
- [x] File reading functionality
- [x] Error handling for invalid arguments and file operations

### 🚧 TODO (Following Rust Book Chapter 12)
- [ ] Extract search logic into separate function
- [ ] Return matching lines instead of entire file content
- [ ] Add case-insensitive search option
- [ ] Environment variable support (IGNORE_CASE)
- [ ] Write output to stderr for errors
- [ ] Add unit tests
- [ ] Refactor into library crate (lib.rs)
- [ ] Improve error handling with custom error types

## Requirements

- Rust (nightly toolchain)
- Cargo

## Project Structure

- [`src/main.rs`](src/main.rs) - Main application logic
- [`Cargo.toml`](Cargo.toml) - Project configuration
- [`story.txt`](story.txt) - Example text file for testing

This project demonstrates fundamental Rust concepts including command-line argument parsing, file I/O, error handling, and struct implementation while following "The Rust Programming Language" book tutorial.