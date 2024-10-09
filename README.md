Time to learn some rust!!

## Installation

To install rust, follow the instructions on the [official rust website](https://www.rust-lang.org/tools/install).

## Running the code

All rust code is in the src/bin directory. To run the code, use the command `cargo run` in the root directory of the project.

Each file in the src directory is a separate rust program. To run a specific program, use the command `cargo run --bin <filename>`.

eg: `cargo run --bin variables`

Each rust program is wrapped inside a `main` function. The `main` function is the entry point of the program.

## Naming conventions

| Convention             | Types that use it                                                               |
| ---------------------- | ------------------------------------------------------------------------------- |
| `snake_case`           | Crates, modules, functions, methods, local variables and parameters, lifetimes. |
| `CamelCase`            | Types (including traits and enums), type parameters in generics.                |
| `SCREAMING_SNAKE_CASE` | Constant and static variables.                                                  |

## Order of learning

1. [Variables](src/bin/variables.rs)
1. [Data types](src/bin/data_types.rs)
1. [Input and output](src/bin/input_output.rs)
1. [Ownership](src/bin/ownership.rs)
