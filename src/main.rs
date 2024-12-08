use arguments::Arguments;
use clap::Parser;
use log::{error, info};
use simple_logger::SimpleLogger;

mod analyzer;
mod arguments;
mod error;
mod normalizer;
mod parser;
mod lexer;

fn main() {
    SimpleLogger::new().init().unwrap();
    let arguments = Arguments::parse();

    match analyzer::analyze_trace(arguments.input, arguments.normalize) {
        Ok(_) => info!("Analyzer could not find a violation"),
        Err(error) => error!("{error}"),
    }
}
