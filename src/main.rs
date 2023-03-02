use clap::{Args, Parser, Subcommand, ValueEnum};
use std::fs;
use std::io::{self, prelude::*};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Sets a custom config file specifying a configuration to use
    #[arg(short, long)]
    config: Option<PathBuf>,
    /// Input neural network
    #[arg(short, long, name = "INPUT")]
    input: Option<PathBuf>,
    /// Output to save to
    #[arg(short, long, name = "OUTPUT")]
    output: Option<PathBuf>,
    #[command(flatten)]
    compress: CompressionParams,
}

#[derive(Default, Parser)]
pub struct CompressionParams {
    #[arg(long)]
    /// Compress floating point values to a lower precision floating point value
    floats: Option<FloatCompression>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
pub enum FloatCompression {
    /// Single precision float (32 bits)
    Single,
    /// Half precision floats (16 bits)
    Half,
}

fn main() {
    let cli = Cli::parse();
}
