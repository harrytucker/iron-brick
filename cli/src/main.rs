use missions_lib;
use missions_lib::*;

use quicli::prelude::*;
use structopt::StructOpt;

/// Manage the iron-brick application
#[derive(Debug, StructOpt)]
struct Cli {
    /// The file to read
    file: String,
    // Quick and easy logging setup you get for free with quicli
    #[structopt(flatten)]
    verbosity: Verbosity,
}

fn main() -> CliResult {
    let args = Cli::from_args();
    args.verbosity.setup_env_logger("head")?;

    let content = read_file(&args.file)?;
    let missions_str = json_to_missions(&content).unwrap();

    println!("{}", missions_to_yaml(&missions_str).unwrap());

    Ok(())
}
