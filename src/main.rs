use std::{ffi::OsStr, path::Path};

mod analyzer;
// mod compiler;
mod executor;
mod parser;
mod symbol_table;
mod types;
fn main() {
    let mut args = std::env::args();
    let current_path = args.next();
    let source_path = args.next();

    if source_path.is_none() {
        return run_interpreter();
    }
    proceess_file(&current_path.unwrap(), &source_path.unwrap());
}

fn proceess_file(_current_path: &str, source_path: &str) {
    const CALC_PREFIX: &str = "ab";
    const OUTPUT_DIR: &str = "output";
    const OUTPUT_FILE_NAME: &str = "out.rs";

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

    let parsed_program;
    match parser::parse_program(&source_code) {
        Ok((rest, syntax_tree)) => {
            let trimmed_rest = rest.trim();
            if trimmed_rest.len() > 0 {
                eprintln!(
                    "Invalid remaining code in '{}': {}",
                    source_path.display(),
                    trimmed_rest
                );
                return;
            }
            parsed_program = syntax_tree;
        }
        Err(err) => {
            eprintln!("Invalid code in '{}': {:?}", source_path.display(), err);
            return;
        }
    }
    eprintln!("parsed program {:?}", parsed_program);
    // return;
    //

    let analyzed_program;
    let mut variables = symbol_table::SymbolTable::new();
    match analyzer::analyze_program(&mut variables, &parsed_program) {
        Ok(analyzed_tree) => {
            analyzed_program = analyzed_tree;
        }
        Err(err) => {
            eprintln!("Invalid code in '{}': {}", source_path.display(), err);
            return;
        }
    }
    eprintln!("analysed program {:?}", parsed_program);

    return;
    // let target_dir = source_path
    //     .parent()
    //     .unwrap_or(Path::new("/"))
    //     .join(OUTPUT_DIR);
    // std::fs::create_dir_all(&target_dir).expect("Cannot create output directory");

    // let output_file_path = target_dir.join(OUTPUT_FILE_NAME);

    // match std::fs::write(
    //     &output_file_path,
    //     compiler::translate_to_rust_program(&variables, &analyzed_program),
    // ) {
    //     Ok(_) => eprintln!(
    //         "Compiled {} to {}.",
    //         source_path.display(),
    //         output_file_path.display()
    //     ),
    //     Err(err) => eprintln!(
    //         "Failed to write to file {}: ({})",
    //         output_file_path.display(),
    //         err
    //     ),
    // }
}

fn run_interpreter() {
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
                    eprintln!("{} : {}", v.0, v.1)
                }
            }

            input => {
                let parsed_program;
                match parser::parse_program(&input) {
                    Ok((rest, syntax_tree)) => {
                        let trimmed_rest = rest.trim();
                        if trimmed_rest.len() > 0 {
                            eprintln!("Unparsed input: `{}`.", trimmed_rest);
                            return;
                        }
                        parsed_program = syntax_tree;
                    }
                    Err(err) => {
                        eprintln!("Error: {:?}", err);
                        return;
                    }
                }

                let analyzed_program;
                match analyzer::analyze_program(&mut variables, &parsed_program) {
                    Ok(analyzed_tree) => {
                        analyzed_program = analyzed_tree;
                        executor::execute_program(&mut variables, &analyzed_program)
                    }
                    Err(err) => {
                        eprintln!("Error: {:?}", err);
                        return;
                    }
                }
            }
        }
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
