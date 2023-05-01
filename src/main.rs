use crate::core::config;
use crate::core::case;
use crate::parsers::pdf;
use crate::utils::specimen;
use std::path::Path;

pub mod core;
pub mod utils;
pub mod parsers;

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
    specimen.load(Path::new(&args.file));

    // Load configuration
    let mut configuration = config::Config::default();
    configuration.load();

    // Create case
    case::create(&specimen.fingerprint.sha256);

    // Parser selection
    match args.parser.to_lowercase().as_str() {
        "pdf" => {
            let mut document = pdf::PdfDocument::default();
            document.parse(&specimen);

        },
        _ => println!("Not implemented"),
    }
}
