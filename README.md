# Autobot

High level scripting language designed for hasle free system automation.

## Syntax:

- `@a < string | number > ` -> declare an identifier and sets them to default value.
- `a := some_expr` -> set value of an identifier
- `>a` -> get value and store it in an identifier
- `< some_expr` -> output an expression or a iddentifier
- `open <string> ` -> open a file or a url
- `on key1 key2 ... [flag] : shortcut_name { valid_autobot_code }` -> runs shortcut code

### Data Types

- strings : Rust String type
- number : Rust f64 type

## Use:

- `cargo r ` =>register daemon
- `cargo r -- source.ab` => Run a .ab file.
- `cargo r -- -c source.ab` => Convert a .ab file into .rs file.
- `cargo r -d ` => Start the daemon to listen for shortcuts

### Debug:

`cargo watch --ignore ./data/* -x "run -- .\data\test.ab" -s "rustc .\data\output\out.rs -o .\data\output\out.exe" -s .\data\output\out.exe`
