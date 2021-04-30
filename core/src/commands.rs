use super::hazard;
use super::error;
use super::shard;

use utils::app_config::AppConfig;
use utils::error::Result;


pub fn run_yshard(outdir: &str, groupby_path: &str) -> Result<()> {
    //TODO: validate inputs

    shard::shard_yaml(outdir, groupby_path).unwrap();

    eprintln!("finished successfully");

    Ok(())
}
