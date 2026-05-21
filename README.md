# sudokuprinter

[![ci](https://github.com/nullnullnullnullnullnullnullnullnullnul/sudokuprinter/actions/workflows/ci.yml/badge.svg)](https://github.com/nullnullnullnullnullnullnullnullnullnul/sudokuprinter/actions/workflows/ci.yml)
[![Rust 2021](https://img.shields.io/badge/Rust-2021-CE422B.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

A Rust CLI that generates sudoku puzzles, persists batches to JSON, and is intended to eventually drive a FutureLogic Gen2 thermal ticket printer over RS232.

## Status

| Component              | State                                          |
| ---------------------- | ---------------------------------------------- |
| Puzzle generator       | implemented (4 difficulties, unique solutions) |
| Backtracking solver    | implemented                                    |
| In-memory queue + JSON | implemented (`save`, `load`, `pop`)            |
| Terminal renderer      | implemented (box-drawing characters)           |
| CLI front end          | implemented (clap-based subcommands)           |
| RS232 / serial driver  | **not yet implemented**                        |
| ESC/P2 raster output   | **not yet implemented**                        |
| Slint or other GUI     | **not yet implemented**                        |

The `print` subcommand currently fails with a clear "not implemented" error. Everything else is functional.

## Build

```bash
cargo build --release
```

## Usage

```text
Usage: sudokuprinter <COMMAND>

Commands:
  generate  Generate a batch of puzzles and write them to a JSON file
  show      Load a JSON batch and pretty-print every puzzle to stdout
  print     Send queued puzzles to the thermal printer (not yet implemented)
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Generate a batch

```bash
./target/release/sudokuprinter generate --count 3 --difficulty hard --output puzzles.json
```

Difficulties: `easy`, `medium`, `hard`, `expert` (case-insensitive). Each maps to a target number of clues remaining after removal:

| Difficulty | Target clues |
| ---------- | -----------: |
| Easy       | 45           |
| Medium     | 35           |
| Hard       | 28           |
| Expert     | 24           |

The actual clue count can be slightly higher than the target when further removal would break the puzzle's unique-solution property.

### Show a batch

```bash
./target/release/sudokuprinter show puzzles.json
```

Drains the queue and renders each puzzle to stdout:

```text
ID: 9866a2e4-b2da-4319-a999-2c111081ddb4
Timestamp: 2026-05-21 23:36:51.170486203 UTC
Difficulty: Easy
┌───────┬───────┬───────┐
│ 1 8 7 │ 5 9 . │ 6 . 3 │
│ . 9 . │ 6 2 3 │ 1 7 . │
...
```

### Print (stub)

```bash
./target/release/sudokuprinter print puzzles.json
# error: printer integration not yet implemented (RS232 + ESC/P2 backend pending)
```

## Quality

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
```

All three are enforced by CI on every PR.

## Project layout

```
src/
  main.rs              # clap CLI + subcommand dispatch
  queue/
    mod.rs             # PrintQueue: generation, persistence, drain
  sudoku/
    mod.rs             # Grid type, Difficulty, terminal renderer
    generator.rs       # generate_sudoku + uniqueness-preserving removal
    solver.rs          # backtracking solver and solution counter
```

## Conventions

- **Commits**: [Conventional Commits](https://www.conventionalcommits.org/) (`feat`, `fix`, `refactor`, `docs`, `chore`, `ci`, `test`, ...).
- **Branching**: `main` is the only long-lived branch. Topic branches land via PR.
- **PRs**: CI must be green. Squash merge.
- **Style**: default `rustfmt` (no unstable config). Run `cargo fmt --all` before pushing.

## Roadmap

In order of intended implementation:

1. RS232 serial driver behind a `Printer` trait so the queue is decoupled from any specific hardware backend (planned crate: `serialport`).
2. ESC/P2 raster bitmap encoder for the FutureLogic Gen2 Universal target (GURUSAGE8, PSA-60-S12RU; 19200 baud, 8-N-1, XON/XOFF; 62mm wide at 203 dpi).
3. ASCII / box-drawing fallback for printers that don't support raster mode.
4. Optional Slint GUI for non-CLI interaction.

Hardware target details preserved here so the connection settings don't get lost between sessions.

## License

MIT. See [LICENSE](LICENSE).
