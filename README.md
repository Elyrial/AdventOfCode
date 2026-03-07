# Advent of Code

Solutions for [Advent of Code](https://adventofcode.com/) puzzles, written in Rust. Covers 2015 to 2025.

## Usage

```bash
./run.sh <year> <day>
```

This fetches the puzzle input automatically (if not already cached) and runs both parts.

## Structure

- `src/solutions/year<YYYY>/day<DD>.rs` -- solution files
- `inputs/year<YYYY>/day<DD>.txt` -- cached puzzle inputs (gitignored)
- `bin/fetch_input.rs` -- input downloader (requires `AOC_SESSION` in `.env`)
