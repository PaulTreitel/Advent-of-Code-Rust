use std::{
    env, fs::{File, OpenOptions}, io::Write, process
};

fn create_file(path: &str) -> Result<File, std::io::Error> {
    OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
}

pub fn handle(year: u32) {
    let success = set_year(year);
    if success.is_err() {
        process::exit(1);
    }
    println!("Set repository to AOC year {}", year);
}

pub fn set_year(year: u32) -> Result<(), ()> {
    let filepath = env::current_dir()
        .unwrap()
        .join("year.txt");
    let mut file = match create_file(filepath.to_str().unwrap()) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to create year file: {e}");
            return Err(());
        }
    };

    match file.write_all(year.to_string().as_bytes()) {
        Ok(()) => (),
        Err(e) => {
            eprintln!("Failed to set year file contents: {e}");
            return Err(());
        }
    }
    Ok(())
}
