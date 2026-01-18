use clap::{Parser};

#[derive(Parser)]
struct Cli {
    cmd: String,
    path: std::path::PathBuf,
}
fn main() {
    let args = Cli::parse();
    let result = std::fs::read_to_string(&args.path);
    let content = match result {
        Ok(content) => {content},
        Err(error) => {panic!("bad file: {}", error)}
    };
    println!("IT WORKS! cmd: {:?} path: {:?}", args.cmd, args.path);

    for line in content.lines() {
        println!("{}", line);
    }

}
