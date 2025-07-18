use ceph_doctor::Result;
use clap::{CommandFactory, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(after_help = "EXAMPLES:
    ceph-doctor monitor                           Monitor cluster with default 5s interval
    ceph-doctor monitor --interval 10            Monitor with 10s interval
    ceph-doctor monitor --prefix-command 'ssh host sudo'  Monitor remote cluster")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Monitor Ceph cluster health and recovery progress in real-time
    Monitor {
        #[arg(long, default_value = "5", help = "Update interval in seconds")]
        interval: u64,
        #[arg(
            long,
            help = "Command prefix for remote execution (e.g., 'ssh host sudo' or 'kubectl exec pod --')"
        )]
        prefix_command: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Monitor {
            interval,
            prefix_command,
        }) => {
            let prefix_args: Option<Vec<String>> = prefix_command
                .as_ref()
                .map(|p| p.split_whitespace().map(|s| s.to_string()).collect());
            ceph_doctor::monitor::run(*interval, prefix_args.as_deref()).await?;
        }
        None => {
            // Print comprehensive help when no subcommand is provided
            Cli::command().print_help()?;
        }
    }

    Ok(())
}
