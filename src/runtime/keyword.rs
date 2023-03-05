#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Keywords {
    Open(OpenKeyword),
    // WriteFile,
}
impl Keywords {
    pub fn open() -> Keywords {
        Keywords::Open(OpenKeyword)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct OpenKeyword;
impl OpenKeyword {
    const NAME: &str = "OpenKeyword";

    pub fn new() -> Self {
        Self
    }
    pub fn execute(self, file_path: &str) -> Result<(), String> {
        println!("Opening : {:?}", &file_path);
        match open::that(&file_path) {
            Err(err) => Err(format!("Failed to open file '{}'\n\n {}", file_path, err)),
            Ok(_) => Ok(()),
        }
    }

    pub fn compile(self, path_arg: &str) -> String {
        format!("{}::new().execute(&{})", Self::NAME, path_arg)
    }
}
