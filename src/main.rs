mod python;

use clap::Parser;
use python::derive_preset;

#[derive(Parser)]
#[command(name = "cpa")]
enum Cli {
    Create(CreateArgs),
    Update(UpdateArgs),
}

#[derive(Parser)]
struct CreateArgs {
    #[clap(long, required = true)]
    name: String,
    #[clap(long, required = false, default_value = "python")]
    preset: String,
}

#[derive(Parser)]
struct UpdateArgs {
    #[clap(long, required = false, default_value = "python")]
    preset: String,
}

fn main() {
    match Cli::parse() {
        Cli::Create(args) => {
            println!("Creating project with name: {}", args.name);
            println!("Using preset: {:?} ", args.preset);
            if args.preset.starts_with("python") {
                derive_preset(args.preset, args.name);
            } else {
                eprintln!("Preset: {:?} not supported currently", args.preset);
            }
        }
        Cli::Update(args) => {
            println!("Updating project cwith preset: {:?}", args.preset);
            eprintln!("Update currently not supported");
        }
    }
}
