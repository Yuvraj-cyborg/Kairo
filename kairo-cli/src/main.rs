use anyhow::Result;
use clap::{Parser,Subcommand};

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Init { dir: Option<String> },
    Build,
    Serve,
    Deploy,
    Mdcheck { dir: String },
}

fn main() -> Result<()> {
    let args = Cli::parse();
    match args.command {
        Command::Init { dir } => {
            let target = dir.unwrap_or_else(|| ".".to_string());
            std::fs::create_dir_all(&target)?;
            let path = std::path::Path::new(&target).join("kairo.toml");
            kairo_core::config::write_default_config(&path)?;
            println!("Wrote default config to {}", path.display());
        }
        Command::Mdcheck { dir } => {
            kairo_core::io::read_markdowns(dir);
            println!("MdCheck succesfull");
        }
        Command::Build => {
    let cfg_path = std::path::Path::new("kairo.toml");
    let cfg = kairo_core::config::load_config(cfg_path)?;
    kairo_core::build::build_site(&cfg)?;
    println!("Build successful");
        }
        Command::Serve => {
            println!("Serving succesfull");
        }
        Command::Deploy => {
            println!("Deploying succesfull");
        }
    }
    Ok(())
}
