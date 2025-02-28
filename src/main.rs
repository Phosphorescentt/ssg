mod fs;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    directory: String,
}

fn main() {
    let args = Args::parse();
    let ft = fs::create_file_tree(args.directory.into());
    println!("{:?}", ft);
}
