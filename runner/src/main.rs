use clap::Parser;
#[derive(Parser, Debug)]
#[command(version = "0.0.1", about = "This program can compress a file.", long_about = None)]
struct Args {
    /// The location of the source file
    #[arg(short, long, help = "Location of the input file")]
    source: String,
    /// Location of the destination file
    #[arg(short, long, help = "Location of the destination file")]
    destination: String,

}
fn main() {
    let args = Args::parse();
    println!("Source: {}", args.source);
    println!("Destination: {}", args.destination);
}
