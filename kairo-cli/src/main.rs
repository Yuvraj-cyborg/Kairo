use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Parser)]
enum Command {
    Init { dir: Option<String> },
    Build,
    Serve,
    Deploy,
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
        Command::Build => {
            println!("Build succesfull");
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
