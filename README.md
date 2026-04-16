# Todo CLI

A tiny terminal todo app.

Simple. Clean. Just enough to practice core Rust concepts.

## What It Does

- Add a task (title + description)
- View all tasks
- Delete a task by ID
- Mark a task done or undone

## Run

```bash
cargo run
```

## Build

```bash
cargo build --release
```

Binary output:

```text
target/release/todo_cli
```

## Notes

- Tasks are kept in memory for now (no file/database storage yet).
- Built with Rust 2024 edition and standard library only.
