# Rust Tutorials Codebase

## Project Overview

This is a **learning-focused Rust repository** containing independent tutorial projects, not a production application. Each subdirectory is a standalone Cargo project demonstrating speci`fic Rust concepts.

## Architecture

- **Independent modules**: Each directory (`guessing_game/`, `ownership/`, `structs/`, etc.) is a separate Cargo workspace with its own `Cargo.toml` and `src/main.rs`
- **No shared code**: Projects don't import from each other - they're educational examples, not interconnected modules
- **Self-contained**: Run projects individually with `cargo run` from their respective directories

## Key Projects

- `common_concepts/`: Variables, data types, functions, control flow, loops, temperature conversion
- `ownership/`: Ownership rules, borrowing, slices with detailed comments
- `guessing_game/`: Interactive CLI with `rand` crate dependency
- `todo-cli/`: File I/O example with `todos.txt` persistence
- `snake_game/`: Game using `macroquad` crate (not actually snake, it's a space shooter)
- `structs/`: Struct definitions, methods, builder pattern (`build_user`)
- `modules/`: Module system basics (`pub mod kitchen`)
- `vectors/`: Mutable vector iteration pattern

## Development Workflow

### Running Projects

```bash
cd <project_name>
cargo run
```

### Dev Watcher (guessing_game only)

The `guessing_game/dev.sh` script provides auto-reload functionality:

- Uses `cargo watch` for file watching
- Type `rs` to manually restart, `q` to quit
- Requires `cargo-watch` installed: `cargo install cargo-watch`

## Coding Conventions

### Edition Configuration

- Most projects use `edition = "2024"` (note: this is unusual, standard is 2021)
- `snake_game` uses `edition = "2021"`

### Code Style Patterns

1. **Educational comments**: Code includes detailed explanations (see `ownership/src/main.rs` lines 13-18)
2. **Function organization**: Multiple example functions called from `main()` to demonstrate concepts
3. **Explicit typing**: Type annotations used for teaching clarity (e.g., `let y: i32 = 10`)
4. **Emoji in output**: CLI tools use emoji for user-friendly messages (`✅`, `📭`, `🧹` in todo-cli)

### Common Patterns

- **Match for error handling**: `match input.trim().parse()` pattern in guessing_game
- **Reference passing**: `&Rectangle` for struct methods without ownership transfer
- **Derive Debug**: Use `#[derive(Debug)]` for structs + `{:#?}` pretty-print
- **Module exports**: Use `pub mod` + `pub fn` for public module APIs

## Documentation Files

- `guessing_game/integers.md`: Integer type reference (ranges, signed vs unsigned formulas)
- Not every project has docs - most are self-explanatory from code

## Building New Examples

When adding new tutorial projects:

1. Create new directory at root level
2. Run `cargo init` inside it
3. Follow naming convention: lowercase with underscores
4. Add focused examples in `main()` - avoid complex abstractions
5. Include inline comments explaining non-obvious Rust concepts
6. Use emoji in CLI output for better UX consistency with existing projects

## Testing

No test files exist - this is a tutorial repo focused on demonstrating syntax and concepts, not TDD practices.
