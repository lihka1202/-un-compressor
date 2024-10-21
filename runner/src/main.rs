use clap::Parser;
#[derive(Parser, Debug)]
#[command(version = "0.0.1", about = "This program can compress a file.", long_about = None)]
struct Args {
    /// The location of the source file, in the input directory for the time being
    #[arg(short, long, help = "Location of the input file")]
    source: String,
    /// Location of the destination file
    #[arg(short, long, help = "Location of the destination file")]
    destination: String,

}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    println!("This is the current directory: {}", std::env::current_dir().unwrap().display().to_string());
    println!("Source: {}", args.source);
    println!("Destination: {}", args.destination);

    let metadata = std::fs::metadata(String::from("input/") + &args.source)?;
    println!("Length before compression: {:?}", metadata.len());

    Ok(())
}
