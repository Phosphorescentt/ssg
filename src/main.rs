mod fs;
mod markdown;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    input_dir: String,
    output_dir: String,
}

fn main() {
    let args = Args::parse();
    let ft = fs::create_file_tree(args.input_dir.into());
    println!("{:?}", ft);
    markdown::render_file_tree(ft, args.output_dir.into());
}
