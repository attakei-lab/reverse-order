use std::env;
use std::fs::copy;
use std::path::PathBuf;
use std::process::exit;

fn main() {
    let mut args = env::args_os();
    if args.len() != 3 {
        exit(1);
    }
    args.next();
    let from_ = PathBuf::from(args.next().unwrap());
    if from_.is_dir() {
        eprintln!("Currently, directory source is not supported.");
        exit(1);
    }
    let to_ = PathBuf::from(args.next().unwrap());
    if !to_.exists() {
        eprintln!("File is not exists.");
        exit(1);
    }
    if to_.is_dir() {
        let to_ = to_.join(from_.file_name().unwrap().to_str().unwrap());
        let result = copy(to_, from_);
        if result.is_ok() {
            exit(0);
        } else {
            eprintln!("{}", result.unwrap_err());
            exit(1);
        }
    } else if to_.is_file() {
        let result = copy(to_, from_);
        if result.is_ok() {
            exit(0);
        } else {
            eprintln!("{}", result.unwrap_err());
            exit(1);
        }
    }
}
