use std::{fs::{File, OpenOptions}, io::Write};

pub mod all;
pub mod download;
pub mod read;
pub mod scaffold;
pub mod solve;
pub mod time;
pub mod new_year;
pub mod set_year;
pub mod attempt;

#[derive(Debug)]
enum WriteError {
    Open,
    Write,
}

fn open_file(filepath: &str) -> Result<File, std::io::Error> {
    OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(filepath)
}

fn write_file(filepath: &str, to_write: &[u8]) -> Result<(), WriteError> {
    let file = open_file(&filepath);
    if file.is_err() {
        eprintln!("Failed to open file {}", filepath);
        return Err(WriteError::Open);
    }
    let mut file = file.unwrap();

    match file.write_all(
        to_write
    ) {
        Ok(()) => Ok(()),
        Err(e) => {
            eprintln!("Failed to write to {filepath}: {e}");
            Err(WriteError::Write)
        }
    }
}
