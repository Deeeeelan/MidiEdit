use clap::Parser;

#[derive(Parser)]
struct Cli {
    cmd: String,
    path: std::path::PathBuf,
}
fn main() {
    let args = Cli::parse();

    println!("IT WORKS! cmd: {:?} path: {:?}", args.cmd, args.path);

    println!("Hello, world!");

}
