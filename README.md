# Data Structures and Algorithms Using Rust

A collection of 100 self-contained Rust exercises covering algorithms and data structures. Each solution lives in a single file using only the standard library (`std`).

Problem statements (in Portuguese) are available in [lista-exercicios-rust.md](lista-exercicios-rust.md).

## Rules

- One file per exercise — no cross-file dependencies
- Standard library only — no external crates
- Comments and docstrings in English

## Project Structure

```
data-structure-and-algorithms/
├── src/
│   ├── main.rs
│   └── bin/
│       ├── algorithms/
│       │   ├── sorting/                    (10 exercises)
│       │   ├── searching/                  (10 exercises)
│       │   ├── recursion-and-backtracking/ (10 exercises)
│       │   └── dynamic-programming/        (10 exercises)
│       └── data_structure/
│           ├── arrays-and-strings/         (10 exercises)
│           ├── linked-lists/               (10 exercises)
│           ├── stacks-and-queues/          (10 exercises)
│           ├── trees/                      (10 exercises)
│           ├── graphs/                     (10 exercises)
│           └── hash-tables/                (10 exercises)
├── Cargo.toml
└── README.md
```

## Topics

| Category | Topic | Files |
|----------|-------|-------|
| algorithms | [sorting](src/bin/algorithms/sorting/) | `question_01.rs` – `question_10.rs` |
| algorithms | [searching](src/bin/algorithms/searching/) | `question_01.rs` – `question_10.rs` |
| algorithms | [recursion-and-backtracking](src/bin/algorithms/recursion-and-backtracking/) | `question_01.rs` – `question_10.rs` |
| algorithms | [dynamic-programming](src/bin/algorithms/dynamic-programming/) | `question_01.rs` – `question_10.rs` |
| data_structure | [arrays-and-strings](src/bin/data_structure/arrays-and-strings/) | `question_01.rs` – `question_10.rs` |
| data_structure | [linked-lists](src/bin/data_structure/linked-lists/) | `question_01.rs` – `question_10.rs` |
| data_structure | [stacks-and-queues](src/bin/data_structure/stacks-and-queues/) | `question_01.rs` – `question_10.rs` |
| data_structure | [trees](src/bin/data_structure/trees/) | `question_01.rs` – `question_10.rs` |
| data_structure | [graphs](src/bin/data_structure/graphs/) | `question_01.rs` – `question_10.rs` |
| data_structure | [hash-tables](src/bin/data_structure/hash-tables/) | `question_01.rs` – `question_10.rs` |

## Running Exercises

### With Cargo

Each exercise is registered as a binary. Run any exercise with:

```bash
cargo run --bin sorting_question_01
cargo run --bin trees_question_05
cargo run --bin hash_tables_question_07
```

Binary naming pattern: `{topic}_question_{XX}` (e.g. `graphs_question_08`, `dynamic_programming_question_03`).

Build all exercises:

```bash
cargo build --bins
```

### With rustc (standalone)

Each file is fully self-contained and can be compiled directly:

```bash
rustc src/bin/algorithms/sorting/question_01.rs && ./question_01
```

## Examples of commits

```
git add . && git commit -m ":rocket: Initial commit." && git push
git add . && git commit -m ":building_construction: Added initial project architecture." && git push
git add . && git commit -m ":building_construction: Update project architecture." && git push
git add . && git commit -m ":memo: Updated project documentation." && git push
git add . && git commit -m ":memo: Updated code documentation." && git push
git add . && git commit -m ":white_check_mark: Added feature xyz." && git push
git add . && git commit -m ":wrench: Fixed xyz usage." && git push
git add . && git commit -m ":heavy_minus_sign: Removed xyz." && git push
git add . && git commit -m ":memo: Adjusted project imports." && git push
git add . && git commit -m ":arrow_up: Updated dependencies." && git push
git add . && git commit -m ":arrow_down: Removed dependencies." && git push
git add . && git commit -m ":wastebasket: Removed unused code." && git push
git add . && git commit -m ":test_tube: Added test functionality xyz." && git push
git add . && git commit -m ":construction_worker: Building in progress." && git push
git add . && git commit -m ":construction_worker: Added CI build system." && git push
```

## License

MIT License

Copyright (c) 2026 William Franco

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

