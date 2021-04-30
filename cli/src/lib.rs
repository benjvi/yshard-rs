use clap::{App, AppSettings, Arg};
use clap::{crate_version, crate_description, crate_authors};

use core::commands;
use utils::app_config::AppConfig;
use utils::error::Result;

/// Match commands
pub fn cli_match() -> Result<()> {
    // Get matches
    let cli_matches = cli_config()?;

    // Merge clap config file if the value is set
    AppConfig::merge_config(cli_matches.value_of("config"))?;

    let groupby_path = cli_matches.value_of("groupby-path").unwrap();
    let outdir = cli_matches.value_of("out").unwrap();

    commands::run_yshard(&outdir, &groupby_path)?;

    Ok(())
}

/// Configure Clap
/// This function will configure clap and match arguments
pub fn cli_config() -> Result<clap::ArgMatches> {
    let cli_app = App::new("rust-starter")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!("\n"))
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .about("Set a custom config file")
                .takes_value(true)
        )
        .arg(
            Arg::new("out")
                .short('o')
                .long("output")
                .about("Directory to place the output files")
                .takes_value(true)
                .required(true)
        )
        .arg(
            Arg::new("groupby-path")
                .short('g')
                .about("Path to the element of the document to group by (in jq syntax)")
                .takes_value(true)
                .required(true)
        );

    // Get matches
    let cli_matches = cli_app.get_matches();

    Ok(cli_matches)
}
