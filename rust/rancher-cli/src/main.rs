// curl -u $TOKEN -X GET -H 'Accept: application/json' -H 'Content-Type: application/json' $URL

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    name: Vec<String>,

    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Get(GetArgs),
    Delete(DeleteArgs),
}

#[derive(Args)]
struct GetArgs {
    name: String,
}

#[derive(Args)]
struct DeleteArgs {
    #[command(subcommand)]
    resource: Resources,
}

#[derive(Subcommand)]
enum Resources {
    /// Use PRTBs
    PRTB,
    /// Use CRTBs
    CRTB,
}

fn main() {
    let cli = Cli::parse();

    println!("name: {:?}", cli.name);
    println!("verbose: {:?}", cli.verbose);

    match &cli.command {
        Commands::Delete(args) => match args.resource {
            Resources::PRTB => {
                println!("PRTB")
            }
            Resources::CRTB => {
                println!("CRTB")
            }
        }
        Commands::Get(args) => {
            println!("Get {}", args.name)
        }
    }
}