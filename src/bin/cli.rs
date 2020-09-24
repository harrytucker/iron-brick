use lib::*;

use failure::Error;
use models::MissionForm;
use quicli::prelude::*;
use structopt::{clap::arg_enum, StructOpt};

// arg_enum! implements FromStr so we can use this as a CLI argument
arg_enum! {
    /// FileFormat represents the file formats the iron-brick cli supports.
    /// If any new file formats need to be supported, they can be added here.
    #[derive(PartialEq, Debug)]
    enum FileFormat {
        Yaml,
        Json,
    }
}

/// You can define command line arguments here, see StructOpt documentation on how to use this.
#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
enum Cli {
    Info {
        /// The missions file to load in, supports either JSON or YAML formats
        #[structopt(default_value = "missions.yaml")]
        file: String,

        /// The format of the missions file to load in
        #[structopt(short, long, default_value = "yaml")]
        format: FileFormat,

        // Quick and easy logging setup you get for free with quicli
        #[structopt(flatten)]
        verbosity: Verbosity,
    },
    Help,
    // Example for when I get to this subcommand
    // Add {
    //     #[structopt(short)]
    //     interactive: bool,
    //     #[structopt(short)]
    //     patch: bool,
    // }
}

/// Main handles command arguments and passesthem to the relevant functions
/// Currently, only the Info command is supported, and prints the maximum score possible for a set of missions
fn main() -> Result<(), Error> {
    let args = Cli::from_args();

    let missions = match args {
        Cli::Help => panic!("hohoh"),
        Cli::Info {
            file,
            format,
            verbosity,
        } => {
            verbosity.setup_env_logger("head")?;
            read_missions(file, format).unwrap()
        }
    };

    println!("Max score: {}", calculate_max_score(&missions));
    Ok(())
}

/// Takes a filename and a FileFormat, currently supports YAML and JSON using Serde.
fn read_missions(
    file: String,
    format: FileFormat,
) -> Result<MissionForm, Box<dyn std::error::Error>> {
    let content = read_file(file)?;

    let missions: MissionForm = match format {
        FileFormat::Yaml => match yaml_to_missions(&content) {
            Ok(mission_form) => mission_form,
            Err(err) => return Err(Box::new(err)),
        },
        FileFormat::Json => match json_to_missions(&content) {
            Ok(mission_form) => mission_form,
            Err(err) => return Err(Box::new(err)),
        },
    };

    Ok(missions)
}

/// Borrows a MissionForm and returns the maximum possible score for that MissionForm
fn calculate_max_score(mission_form: &MissionForm) -> i32 {
    let mut max_score = 0;

    for mission in &mission_form.missions {
        for field in &mission.fields {
            match field {
                models::Field::StringField(_) => { /*do nothing*/ }
                models::Field::CheckboxField(field) => max_score = max_score + field.value,
                models::Field::SelectField(field) => {
                    for selection in &field.choices {
                        max_score = max_score + selection.value;
                    }
                }
                models::Field::RadioField(field) => {
                    for selection in &field.choices {
                        max_score = max_score + selection.value;
                    }
                }
            }
        }
    }

    max_score
}
