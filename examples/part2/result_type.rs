use std::{fs, io};

#[derive(Debug)]
enum ProcessError {
    IoError(io::Error),
    EmptyFile,
}

impl From<io::Error> for ProcessError {
    fn from(error: io::Error) -> Self {
        ProcessError::IoError(error)
    }
}

fn read_file(filename: &str) -> Result<String, ProcessError> {
    let content = fs::read_to_string(filename)?; // ? operator propagates errors

    if content.is_empty() {
        return Err(ProcessError::EmptyFile);
    }

    Ok(content)
}

fn process() {
    match read_file("data.txt") {
        Ok(content) => println!("File content: {}", content),
        Err(ProcessError::IoError(e)) => eprintln!("IO Error: {}", e),
        Err(ProcessError::EmptyFile) => eprintln!("Error: File is empty"),
    }
}

fn main() {
    process()
}
