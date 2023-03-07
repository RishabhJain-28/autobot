use std::{ffi::OsStr, path::Path};

use crate::{parser::ParsedProgram, symbol_table::SymbolTable};

mod analyzer;
mod compiler;
mod executor;
mod parser;
mod runtime;
mod shortcuts;
mod symbol_table;

fn main() {
    let mut args = std::env::args();

    let current_path = args.next();

    let flag_or_source = args.next();
    if flag_or_source.is_none() {
        return run_interpreter();
    }
    let flag_or_source = flag_or_source.unwrap();

    match flag_or_source.trim() {
        "-d" => {
            println!("daemon mode only");
        }
        "-c" => {
            println!("compile mode");
            let source = args.next();
            compile_to_rust(&current_path.unwrap(), &source.unwrap());
        }
        _ => {
            //TODO: create file executor
            unimplemented!()
        }
    }
}

fn compile_to_rust(_current_path: &str, source_path: &str) {
    const CALC_PREFIX: &str = "ab";
    const OUTPUT_DIR: &str = "output";
    const OUTPUT_FILE_NAME: &str = "main.rs";
    let source_path = Path::new(source_path);
    let source_ext = source_path.extension().unwrap_or(OsStr::new(CALC_PREFIX));

    if source_ext != CALC_PREFIX {
        return eprintln!(
            "Invalid argument {}, file must end with {}",
            source_path.display(),
            CALC_PREFIX
        );
    }

    let source_code = std::fs::read_to_string(source_path);

    if source_code.is_err() {
        return eprintln!(
            "Error reading file {}\n {}",
            source_path.display(),
            source_code.unwrap_err()
        );
    }

    let source_code = source_code.unwrap();

    let parsed_program = parser::parse_program(&source_code);

    if parsed_program.is_err() {
        eprintln!(
            "Invalid code in '{}': \n\nErr: {:?}",
            source_path.display(),
            parsed_program.unwrap_err()
        );
        return;
    }
    let (rest, syntax_tree) = parsed_program.unwrap();
    let trimmed_rest = rest.trim();
    if trimmed_rest.len() > 0 {
        eprintln!(
            "Invalid code in '{}': \n\nErr: {}",
            source_path.display(),
            trimmed_rest
        );
        return;
    }
    let mut variables = symbol_table::SymbolTable::new();
    let analyzed_program = analyzer::analyze_program(&mut variables, &syntax_tree);
    if analyzed_program.is_err() {
        eprintln!(
            "Invalid code in '{}': \n\nErr: {}",
            source_path.display(),
            analyzed_program.unwrap_err()
        );
        return;
    }
    let analyzed_program = analyzed_program.unwrap();
    let target_dir = source_path
        .parent()
        .unwrap_or(Path::new("/"))
        .join(OUTPUT_DIR);
    std::fs::create_dir_all(&target_dir.join("src")).expect("Cannot create output directory");

    // let out_dir = Path::new(env!("OUT_DIR"));

    // copy_cargo_file(&out_dir.join("cargo.toml"), &target_dir);
    // std::fs::copy(&out_dir.join("cargo.toml"), &target_dir)
    //     .expect("Cannot copy dependency: cargo.toml");
    let output_file_path = target_dir.join("src").join(OUTPUT_FILE_NAME);
    let cargo_output_file_path = target_dir.join("Cargo.toml");

    match std::fs::write(
        &output_file_path,
        compiler::translate_to_rust_program(&variables, &analyzed_program),
    ) {
        Ok(_) => eprintln!(
            "Compiled {} to {}.",
            source_path.display(),
            output_file_path.display()
        ),
        Err(err) => eprintln!(
            "Failed to write to file {}: ({})",
            output_file_path.display(),
            err
        ),
    }
    match std::fs::write(&cargo_output_file_path, get_cargo()) {
        Ok(_) => (),
        Err(err) => eprintln!(
            "Failed to write to file {}: ({})",
            output_file_path.display(),
            err
        ),
    }
    // fn copy_cargo_file(source: impl AsRef<Path>, destination: impl AsRef<Path>) {
    //     let mut dir = fs::read_dir(source).unwrap();
    //     let file = dir.find(|entry| {
    //         entry
    //             .as_ref()
    //             .unwrap()
    //             .file_name()
    //             .into_string()
    //             .unwrap()
    //             .contains("Cargo.toml")
    //     });
    //     let file = file.unwrap().unwrap();
    //     fs::copy(file.path(), destination.as_ref().join(file.file_name())).unwrap();
    // }
}

fn run_interpreter() {
    //TODO : start the daemon

    eprintln!("* Calc interactive interpreter *");
    let mut variables = symbol_table::SymbolTable::new();
    loop {
        let cmd: String = input_command();
        match cmd.trim() {
            "q" | "exit" | "quit" => break,
            "c" | "clear" => {
                variables = symbol_table::SymbolTable::new();
                eprintln!("Cleared Variables");
            }
            "v" | "variables" => {
                eprintln!("Variables:");
                for v in variables.iter() {
                    eprintln!("{} => {:?}", v.0, v.1)
                }
            }

            input => parse_input(&mut variables, input),
        }
    }
    fn input_command() -> String {
        let mut text = String::new();
        eprint!("> ");
        std::io::stdin()
            .read_line(&mut text)
            .expect("Cannot read line.");
        text
    }

    fn parse_input(variables: &mut SymbolTable, input: &str) {
        match parser::parse_program(&input) {
            Ok((rest, syntax_tree)) => {
                let trimmed_rest = rest.trim();
                if trimmed_rest.len() > 0 {
                    eprintln!("Unparsed input: `{}`.", trimmed_rest);
                }

                execute_parsed_program(variables, syntax_tree);
            }
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        }
    }
    fn execute_parsed_program(variables: &mut SymbolTable, parsed_program: ParsedProgram) {
        match analyzer::analyze_program(variables, &parsed_program) {
            Ok(analyzed_tree) => {
                match executor::execute_program(variables, &analyzed_tree) {
                    Ok(_) => (),
                    Err(err) => {
                        eprintln!("Error: {}", err);
                    }
                };
            }
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        }
    }
}

fn get_cargo() -> String {
    format!(
        r#"
[package]
name = "autobot"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
nom = "7.1.3"
open = "3.2.0"

    "#
    )
}
