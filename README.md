# Autobot

High level scripting language designed for hasle free system automation.

## Syntax:

- `@a < string | number > ` -> declare an identifier and sets them to default value.
- `a := some_expr` -> set value of an identifier
- `>a` -> get value and store it in an identifier
- `< some_expr` -> output an expression or a iddentifier

### Data Types

- strings : Rust String type
- number : Rust f64 type

## Use:

- `cargo r -- source.ab` => Convert a .ab file into .rs file.

- `cargo r ` => If no file name is supplied , the interpreter mode starts.

### Debug:

`cargo watch --ignore ./data/* -x "run -- .\data\test.ab" -s "rustc .\data\output\out.rs -o .\data\output\out.exe" -s .\data\output\out.exe`
