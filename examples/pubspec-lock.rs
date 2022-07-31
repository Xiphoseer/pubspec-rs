use std::path::PathBuf;

use clap::Parser;
use pubspec::LockFile;

#[derive(Parser)]
struct Options {
    path: PathBuf,
}

fn main() {
    let opts = Options::parse();
    let text = std::fs::read_to_string(&opts.path).unwrap();
    let data = serde_yaml::from_str::<LockFile>(&text).unwrap();
    println!("{:#?}", data);
}
