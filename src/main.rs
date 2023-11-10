mod python;

use std::process;

use clap::Parser;
use python::setup_preset;
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
    #[clap(long, required = false, default_value = "python3.10")]
    preset: String,
}

fn check_pyver(preset: &str) {
    let re = Regex::new(r"python(3\.\d+|4\.\d+)").unwrap();
    if re.captures(preset).is_none() {
        eprintln!("Python version not recognized in --preset, invalid input. Expected format: 'python3.xx'");
        process::exit(1);
    };
}

fn main() {
    match Cli::parse() {
        Cli::Create(args) => {
            println!("Creating project with name: {}", args.name);
            println!("Using preset: {:?} ", args.preset);
            if args.preset.starts_with("python") {
                check_pyver(&args.preset);
                setup_preset(&args.preset, args.name, true);
            } else {
                eprintln!("Preset: {:?} not supported", args.preset);
            }
        }
        Cli::Update(args) => {
            println!("Updating project with preset: {:?}", args.preset);
            check_pyver(&args.preset);
            setup_preset(&args.preset, "".to_string(), false);
        }
    }
}
