use std::{io::Write, str::FromStr};

pub enum LoggerComponent {
    Daemon,
}

impl LoggerComponent {
    pub fn value(&self) -> &str {
        match self {
            LoggerComponent::Daemon => "DAEMON",
        }
    }
}

impl LoggerComponent {
    pub fn log_error<Err: std::error::Error>(self, error: Err) {
        let comp = self.value();
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open("D:/Projects/Rust/Rust Projects/autobot/autobot_logs.txt")
            .unwrap();
        f.write_all(String::from(format!("[ERROR: {comp}]: {error}\n")).as_bytes())
            .unwrap();
    }
    pub fn log(self, text: &str, new_file: Option<bool>) {
        //TODO imporve
        let comp = self.value();

        let mut f = std::fs::OpenOptions::new()
            .truncate(!new_file.unwrap_or(true))
            .write(true)
            .append(new_file.unwrap_or(true))
            .create(true)
            .open("D:/Projects/Rust/Rust Projects/autobot/autobot_logs.txt")
            .unwrap();
        f.write_all(String::from(format!("[LOG: {comp}]: {text}\n")).as_bytes())
            .unwrap();
    }
}
