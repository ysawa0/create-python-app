mod presets;

use std::process;

use clap::Parser;
use presets::{common, python, rust};
use regex::Regex;

#[derive(Parser)]
#[clap(
    name = "cpa",
    version,
    about = "CPA helps you set up new projects, ultra fast",
    long_about = "CPA is a cli tool designed to expedite the setup of new projects by automating the creation of various configuration files."
)]
enum Cli {
    #[clap(about = "Create a new project", long_about = "Create a new project using specified preset.")]
    Create(CreateArgs),
    #[clap(
        about = "Update existing project",
        long_about = "Overwrite existing current working dir with CPA preset."
    )]
    Update(UpdateArgs),
}

#[derive(Parser)]
struct CreateArgs {
    #[clap(long, required = true)]
    name: String,
    #[clap(long, required = false, default_value = "python3.10")]
    preset: String,
}

#[derive(Parser)]
struct UpdateArgs {
    #[clap(long, required = true)]
    name: String,
    #[clap(long, required = false, default_value = "python3.10")]
    preset: String,
}

pub struct Language {
    language: String,
    ver: String,
}

#[allow(clippy::needless_return)]
fn validate_preset(preset: &str) -> Language {
    if preset == "rust" {
        return Language {
            language: "rust".to_string(),
            ver: "".to_string(),
        };
    }

    let re = Regex::new(r"python(3\.\d+|4\.\d+)").unwrap();
    if let Some(caps) = re.captures(preset) {
        return Language {
            language: "python".to_string(),
            ver: caps[1].to_string(),
        };
    } else {
        eprintln!("Python version not recognized in --preset, invalid input. Expected format: 'python3.xx'");
        process::exit(1);
    }
}

fn main() {
    match Cli::parse() {
        Cli::Create(args) => {
            println!("Creating project with name: {}", args.name);
            println!("Using preset: {:?} ", args.preset);
            let lang = validate_preset(&args.preset);
            let create = true;
            if lang.language == "python" {
                let prefix = common(&args.name, create, &lang);
                python(&args.name, &prefix, &lang);
            } else if lang.language == "rust" {
                let prefix = common(&args.name, create, &lang);
                rust(&args.name, &prefix);
            } else {
                eprintln!("Preset: {:?} not supported", args.preset);
            }
        }
        Cli::Update(args) => {
            println!("Updating project with name: {}", args.name);
            println!("Using preset: {:?} ", args.preset);
            let lang = validate_preset(&args.preset);
            let create = false;
            if lang.language == "python" {
                let prefix = common(&args.name, create, &lang);
                python(&args.name, &prefix, &lang);
            } else if lang.language == "rust" {
                let prefix = common(&args.name, create, &lang);
                rust(&args.name, &prefix);
            } else {
                eprintln!("Preset: {:?} not supported", args.preset);
            }
        }
    }
}
