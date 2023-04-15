use crate::utils::specimen;

pub mod utils;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    author="Roger C.B. Johnsen", 
    version="0.1-Alpha", 
    about="Dissection tool for maldocs", 
    long_about = None
)]
struct Args {
    #[arg(short, long)]
    file: String,

    #[arg(short, long)]
    parser: String,
}

fn main() {
    let args = Args::parse();

    // In our sphere each individual file loaded is called a specimen. 
    // All files are loaded in the same way prior to running respective
    // parser.
    let mut specimen = specimen::Specimen::default();
    specimen.load(&args.file);

    // Parser selection
    match args.parser.to_lowercase().as_str() {
        "pdf" => println!("PDF Parser selected."),
        _ => println!("Not implemented"),
    }
}
