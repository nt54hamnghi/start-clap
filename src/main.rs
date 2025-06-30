use clap::Parser;
use {{crate_name}}::cli::Cli;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    Ok(())
}
