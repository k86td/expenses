use clap::Parser;
use expenses::{cli::Cli, models::CliContext, sqlite::SqliteRepository, Result};

fn main() -> Result<()> {
    let cli = Cli::parse();

    let repo = SqliteRepository::initialize("testing.db")?;
    let ctx = CliContext {
        repo: &repo,
        termsize: termsize::get().unwrap(),
    };

    cli.run(ctx)?;

    Ok(())
}
