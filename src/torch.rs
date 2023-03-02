//! Operations on torch networks
//!
//! Is this done using VarStore?
//!
use crate::*;
use eyre::Result;
use std::path::Path;
use tch::{nn::VarStore, Device, TrainableCModule};

pub fn optimise(input: &Path, output: &Path, params: CompressionParams) -> Result<()> {
    let device = Device::cuda_if_available();
    let mut res = VarStore::new(device);

    let model = TrainableCModule::load(input, res.root())?;

    println!("{:?}", res);

    match params.floats {
        Some(FloatCompression::Single) => {
            res.float();
        }
        Some(FloatCompression::Half) => {
            res.half();
        }
        Some(FloatCompression::BFloat16) => {
            res.bfloat16();
        }
        None => {}
    }
    model.save(output)?;
    Ok(())
}
