use clap::{Args, Parser, Subcommand, ValueEnum};
use eyre::{eyre, Result};
use std::fs;
use std::io::{self, prelude::*};
use std::path::PathBuf;

#[cfg(feature = "tensorflow")]
mod tf;

#[cfg(feature = "tch")]
mod torch;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Sets a custom config file specifying a configuration to use
    #[arg(short, long)]
    config: Option<PathBuf>,
    /// Input neural network
    #[arg(short, long, name = "INPUT")]
    input: PathBuf,
    /// Output to save to
    #[arg(short, long, name = "OUTPUT")]
    output: Option<PathBuf>,
    #[command(flatten)]
    compress: CompressionParams,
    #[arg(short, long)]
    engine: Engine,
}

#[derive(Debug, Default, Parser)]
pub struct CompressionParams {
    #[arg(long)]
    /// Compress floating point values to a lower precision floating point value
    floats: Option<FloatCompression>,
}

/// Compress a floating point number (TODO think about fixed point)
#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
pub enum FloatCompression {
    /// Single precision float (32 bits)
    Single,
    /// Half precision floats (16 bits)
    Half,
    /// BFloat16 (16 bit 3 more bits in exponent than a half)
    BFloat16,
}

/// Compress a floating point number (TODO think about fixed point)
#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
pub enum Engine {
    /// Tensorflow neural network
    Tensorflow,
    /// Torch neural network
    Torch,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    println!("Running: {:?}", cli);

    let output = match &cli.output {
        Some(s) => s.clone(),
        None => PathBuf::from(format!("{}.new", cli.input.display())),
    };

    match cli.engine {
        Engine::Torch => {
            cfg_if::cfg_if! {
                if #[cfg(feature="tch")] {
                    torch::optimise(&cli.input, &output, cli.compress)?;
                } else {
                    return Err(eyre!("The munger is built without torch support"));
                }
            }
        }
        Engine::Tensorflow => {
            cfg_if::cfg_if! {
                if #[cfg(feature="tensorflow")] {

                } else {
                    return Err(eyre!("The munger is built without torch support"));
                }
            }
        }
    }

    Ok(())
}
