use clap::{Parser, Subcommand};
use ceph_doctor::Result;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Monitor {
        #[arg(long, default_value = "5")]
        interval: u64,
        #[arg(long, help = "Test mode using sample JSON files")]
        test: bool,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Monitor { interval, test } => {
            if *test {
                ceph_doctor::monitor::run_test(*interval).await?;
            } else {
                ceph_doctor::monitor::run(*interval).await?;
            }
        }
    }

    Ok(())
}