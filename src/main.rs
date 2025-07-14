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
        #[arg(long, help = "Command prefix for remote execution (e.g., 'ssh host sudo' or 'kubectl exec pod --')")]
        prefix_command: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Monitor { interval, test, prefix_command } => {
            if *test {
                ceph_doctor::monitor::run_test(*interval).await?;
            } else {
                let prefix_args: Option<Vec<String>> = prefix_command.as_ref().map(|p| 
                    p.split_whitespace().map(|s| s.to_string()).collect()
                );
                ceph_doctor::monitor::run(*interval, prefix_args.as_deref()).await?;
            }
        }
    }

    Ok(())
}