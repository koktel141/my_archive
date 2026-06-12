use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process;

mod crypto;
mod pack;
mod unpack;

#[derive(Parser)]
#[command(name = "my_archive", version = "1.0", about = "Archive and encrypt files")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    
    Pack {
        
        #[arg(long)]
        password: Option<String>,

        
        #[arg(required = true)]
        files: Vec<PathBuf>,
    },
    
    Unpack {
        
        #[arg(long)]
        password: Option<String>,

        
        archive_path: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match &cli.command {
        Commands::Pack { password, files } => {
            pack::pack(files, password.as_deref())
        }
        Commands::Unpack { password, archive_path } => {
            unpack::unpack(archive_path, password.as_deref())
        }
    };

    if let Err(e) = result {
        eprintln!("{}", e);
        process::exit(1);
    }
}
