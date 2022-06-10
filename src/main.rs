use std::{ffi::OsStr, fs::File, io::{BufRead, BufReader}};

use argparse::{parser::ArgumentParser, Store};
use walkdir::{DirEntry, WalkDir};


fn main() {
    let mut start_dir = String::new();
    let mut expected_ext = String::new();

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Recursive line counter");
        ap.refer(&mut start_dir)
            .add_option(&["-d", "--dir"], Store, "Path to start directory")
            .required();
        ap.refer(&mut expected_ext)
            .add_option(&["-e", "--ext"], Store, "Extension to search")
            .required();
        ap.parse_args_or_exit();
    }

    count_lines_rec(&start_dir, &expected_ext);
}

fn count_lines_rec(start_dir: &str, expected_ext: &str) {
    let walker = WalkDir::new(start_dir).into_iter();
    for entry in walker.filter_entry(|entry| valid_ext_or_dir(entry, expected_ext)) {
        let entry = entry.expect("Start directory doesn't exist.");
        if entry.file_type().is_file() {
            let file = BufReader::new(
                File::open(entry.path()).expect("Unable to open file.")
            );
            println!("{}: {} lines", entry.path().display(), file.lines().count());
        }
    }
}

fn valid_ext_or_dir(entry: &DirEntry, expected_ext: &str) -> bool {
    match entry.path().extension().and_then(OsStr::to_str) {
        Some(entry_ext) => entry_ext == expected_ext,
        None => entry.file_type().is_dir(),
    }
}
