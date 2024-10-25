use clap::{Args, Parser, Subcommand};
use serde_json::json;

use crate::{models::CliContext, repository::ExpensesRepository, Error};

#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Add a new expense
    Add(AddExpense),
}

impl Cli {
    pub fn match_command(self) -> impl ProcessCommand {
        match self.command {
            Commands::Add(v) => v,
        }
    }
}

#[derive(Args, Debug)]
pub struct AddExpense {
    /// Value of the expense
    pub value: f32,
}

pub trait ProcessCommand {
    /// Process the command
    fn process<R>(&self, ctx: CliContext<R>) -> Result<(), Error>
    where
        R: ExpensesRepository;
}

/// This adds a new expense using the ExpensesRepository
impl ProcessCommand for AddExpense {
    fn process<R>(&self, ctx: CliContext<R>) -> Result<(), Error>
    where
        R: ExpensesRepository,
    {
        let now = chrono::Utc::now();
        ctx.repo.create(crate::models::Expense {
            uuid: uuid::Uuid::new_v4().to_string(),
            created: now,
            modified: now,
            data: json!({"value": self.value}),
        })?;

        Ok(())
    }
}
