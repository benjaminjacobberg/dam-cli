//! DAM CLI - Docker management for OpenCode
//!
//! A simple CLI to build, run, and teardown OpenCode containers.

use anyhow::Result;
use clap::{Parser, Subcommand};

/// Options for the build command
#[derive(Parser)]
#[command(next_line_help = true)]
struct BuildOpts {
    #[arg(short, long, default_value = "java")]
    stack: String,
}

/// DAM CLI - Manage OpenCode Docker containers
#[derive(Parser)]
#[command(name = "dam")]
#[command(version = "0.1.0")]
#[command(long_about = "Build, run, and teardown OpenCode containers")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build the Docker image
    Build(BuildOpts),

    /// Run the container interactively
    Run,

    /// Run as ACP server for Zed/Claude connection
    Acp,

    /// Stop and remove running containers
    Teardown,

    /// Show status of containers
    Status,

    /// Debug: show loaded directory
    Debug,

    /// Open the OpenCode web docs
    Web,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Paths relative to the dam-cli directory
    let stacks_path = std::path::Path::new("./stacks").to_path_buf();
    let dockerfile_path = std::path::Path::new("./Dockerfile").to_path_buf();
    let build_context = std::path::Path::new(".").to_path_buf();

    let service = dam_cli::CliApplicationService::new(stacks_path, dockerfile_path, build_context);

    match cli.command {
        Commands::Build(opts) => service.build(&opts.stack)?,
        Commands::Run => service.run()?,
        Commands::Acp => service.acp()?,
        Commands::Teardown => service.teardown()?,
        Commands::Status => service.status()?,
        Commands::Debug => service.debug()?,
        Commands::Web => service.web()?,
    }

    Ok(())
}
