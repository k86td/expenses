use clap::Parser;
use expenses::{cli::Cli, models::CliContext, sqlite::SqliteRepository, Result};

fn main() -> Result<()> {
    let cli = Cli::parse();

    let repo = SqliteRepository::open("testing.db").unwrap();
    let ctx = CliContext { repo: &repo };

    cli.run(ctx)?;

    Ok(())
}
