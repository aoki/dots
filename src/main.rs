use clap::Parser;

#[derive(Parser)]
#[clap(version, about)]
struct Cli {}

fn main() {
    let _ = Cli::parse();
    println!("Hello, world!");
}
