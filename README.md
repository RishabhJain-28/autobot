# Calc Compiler

## Calc language syntax:

_only support f64 data type._

- `@a` -> declare an identifier and sets them to 0.0.
- `a := some_expr` -> set value of an identifier
- `>a` -> get value and store it in an identifier
- `< some_expr` -> output an expression or a iddentifier

## Use:

- `cargo r -- source.calc` Convert a .calc file into .rs file.

- `cargo r ` If no file name is supplied , the interpreter mode starts.
