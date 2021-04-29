#[macro_use]
extern crate log;

pub mod commands;
pub mod hazard;
pub mod error;
pub mod shard;

use utils::error::Result;

pub fn start() -> Result<()> {
    // does nothing

    Ok(())
}
