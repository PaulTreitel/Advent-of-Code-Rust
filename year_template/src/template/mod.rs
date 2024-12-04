use std::{env, fs, path::PathBuf, str::FromStr};

pub mod aoc_cli;
pub mod runner;

pub use day::*;

mod day;

pub const ANSI_ITALIC: &str = "\x1b[3m";
pub const ANSI_BOLD: &str = "\x1b[1m";
pub const ANSI_RESET: &str = "\x1b[0m";

/// Helper function that reads a text file to a string.
#[must_use]
pub fn read_file(folder: &str, day: Day) -> String {
    let cwd = PathBuf::from_str(env!("CARGO_MANIFEST_DIR")).unwrap();
    let filepath = cwd
        .join("data")
        .join(folder)
        .join(format!("{day}.txt"));
    let f = fs::read_to_string(filepath);
    f.expect("could not open input file")
}

/// Helper function that reads a text file to string, appending a part suffix. E.g. like `01-2.txt`.
#[must_use]
pub fn read_file_part(folder: &str, day: Day, part: u8) -> String {
    let cwd = PathBuf::from_str(env!("CARGO_MANIFEST_DIR")).unwrap();
    let filepath = cwd
        .join("data")
        .join(folder)
        .join(format!("{day}-{part}.txt"));
    let f = fs::read_to_string(filepath);
    f.expect("could not open input file")
}

pub fn get_year() -> Result<u32, ()> {
    let mut buf = String::new();
    let filepath = env::current_dir()
        .unwrap()
        .join("year.txt");
    let yearfile = fs::OpenOptions::new()
        .read(true)
        .open(filepath);
    match yearfile {
        Ok(mut file) => {
            if let Ok(_) = std::io::Read::read_to_string(&mut file, &mut buf) {
                buf.parse::<u32>().or(Err(()))
            } else {
                Err(())
            }
        }
        Err(_) => {
            Err(())
        }
    }
}

pub fn get_year_exit_on_fail() -> u32 {
    let year = get_year();
    if year.is_err() {
        eprintln!("{}", crate::YEAR_NOT_FOUND_ERROR_MSG);
        std::process::exit(1);
    }
    year.unwrap()
}

/// Creates the constant `DAY` and sets up the input and runner for each part.
///
/// The optional, second parameter (1 or 2) allows you to only run a single part of the solution.
#[macro_export]
macro_rules! solution {
    ($day:expr) => {
        $crate::solution!(@impl $day, [part_one, 1] [part_two, 2]);
    };
    ($day:expr, 1) => {
        $crate::solution!(@impl $day, [part_one, 1]);
    };
    ($day:expr, 2) => {
        $crate::solution!(@impl $day, [part_two, 2]);
    };

    (@impl $day:expr, $( [$func:expr, $part:expr] )*) => {
        /// The current day.
        const DAY: $crate::template::Day = $crate::day!($day);

        #[cfg(feature = "dhat-heap")]
        #[global_allocator]
        static ALLOC: dhat::Alloc = dhat::Alloc;

        fn main() {
            use $crate::template::runner::*;
            let input = $crate::template::read_file("inputs", DAY);
            $( run_part($func, &input, DAY, $part); )*
        }
    };
}
