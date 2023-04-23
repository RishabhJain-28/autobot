use std::os::windows;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Copy, Deserialize, Serialize)]
pub enum Keywords {
    Open(OpenKeyword),
    // WriteFile,
}
impl Keywords {
    pub fn open() -> Keywords {
        Keywords::Open(OpenKeyword)
    }
}

pub trait Keyword<Args> {
    fn execute_keyword(&self, _: Args) -> Result<(), String> {
        unimplemented!()
    }
    fn compile_keyword(&self, _: Args) -> String {
        unimplemented!()
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct OpenKeyword;
impl OpenKeyword {
    const NAME: &str = "OpenKeyword";
    // pub fn new() -> Self {
    //     Self
    // }
}
impl Keyword<&str> for OpenKeyword {
    fn execute_keyword(&self, path_arg: &str) -> Result<(), String> {
        println!("Opening : {:?}", &path_arg);
        match open::that(&String::from(path_arg)) {
            Err(err) => Err(format!("Failed to open file '{}'\n\n {}", path_arg, err)),
            Ok(_) => Ok(()),
        }
    }

    fn compile_keyword(&self, path_arg: &str) -> String {
        format!("{}::new().execute(&{})", Self::NAME, path_arg)
    }
}
