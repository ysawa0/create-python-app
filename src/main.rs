mod python;

use clap::Parser;
use python::setup_preset;

#[derive(Parser)]
#[command(name = "cpa", version)]
enum Cli {
    Create(CreateArgs),
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

fn main() {
    match Cli::parse() {
        Cli::Create(args) => {
            println!("Creating project with name: {}", args.name);
            println!("Using preset: {:?} ", args.preset);
            if args.preset.starts_with("python") {
                setup_preset(args.preset, args.name, true);
            } else {
                eprintln!("Preset: {:?} not supported yet", args.preset);
            }
        }
        Cli::Update(args) => {
            println!("Updating project with preset: {:?}", args.preset);
            setup_preset(args.preset, "".to_string(), false);
            // eprintln!("Update not yet implemented");
        }
    }
}
