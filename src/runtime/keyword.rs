#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Keyword {
    Open,
    // WriteFile,
}

impl Keyword {
    pub fn open(file_path: &str) -> Result<(), String> {
        println!("Opening : {:?}", &file_path);
        match open::that(&file_path) {
            Err(err) => Err(format!("Failed to open file '{}'\n\n {}", file_path, err)),
            Ok(_) => Ok(()),
        }
    }
}
