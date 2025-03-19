mod fs;
mod markdown;
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    input_dir: String,
    output_dir: String,
}

fn main() {
    let args = Args::parse();
    let _ = fs::prep_output_dir(args.output_dir.clone().into());
    println!("Cleaned output dir.");
    let ft = fs::create_file_tree(args.input_dir.into());
    println!("Parsed file tree.");
    let output_dir = PathBuf::from(args.output_dir);
    let _ = markdown::render_file_tree(ft, &output_dir);
    println!("Site generated!")
}
