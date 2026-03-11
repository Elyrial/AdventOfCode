# Advent of Code

Solutions for [Advent of Code](https://adventofcode.com/) puzzles, written in Rust. Covers 2015 to 2025. This project is still in progress.

## Usage

```bash
./run.sh <year> <day>
```

This fetches the puzzle input automatically (if not already cached) and runs both parts.

## Testing

```bash
cargo test                        # run all tests
cargo test year2025               # run tests for a specific year
cargo test year2025::day08        # run tests for a specific day
```

## Structure

- `src/solutions/year<YYYY>/day<DD>.rs` -- solution files
- `inputs/year<YYYY>/day<DD>.txt` -- cached puzzle inputs (gitignored)
- `bin/fetch_input.rs` -- input downloader (requires `AOC_SESSION` in `.env`)
