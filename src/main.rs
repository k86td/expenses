use std::{error::Error, sync::Arc};

use clap::Parser;
use expenses::{
    cli::{self, AddExpense, Cli, Commands, ProcessCommand},
    models::CliContext,
    repository::ExpensesRepository,
    sqlite::SqliteRepository,
};

fn main() -> Result<(), impl Error> {
    let cli = Cli::parse();
    let command = cli.match_command();

    let repo = SqliteRepository::initialize(":memory:")?;
    let ctx = CliContext { repo: &repo };

    command.process(ctx)?;

    dbg!(repo.get_all(5)?);

    Ok::<(), rusqlite::Error>(())
}
