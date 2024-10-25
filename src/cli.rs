use clap::{Args, Parser, Subcommand};
use serde_json::json;
use tabled::{settings::Width, Table};

use crate::{error::AppError, models::CliContext, repository::ExpensesRepository, Error};

#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Add a new expense
    Add(AddExpense),
    /// Show a raw table of all expenses
    Raw(ShowExpenseTable),
    /// Delete an expense using its UUID. Minimum length for the UUID
    /// is 8 characters
    Delete(DeleteExpense),
}

impl Cli {
    pub fn run<R>(self, ctx: CliContext<R>) -> crate::Result<()>
    where
        R: ExpensesRepository,
    {
        match self.command {
            Commands::Add(v) => {
                v.process(ctx)?;
                Ok(())
            }
            Commands::Raw(s) => {
                s.process(ctx)?;
                Ok(())
            }
            Commands::Delete(d) => {
                d.process(ctx)?;
                Ok(())
            }
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

#[derive(Args, Debug)]
pub struct ShowExpenseTable {}

impl ProcessCommand for ShowExpenseTable {
    fn process<R>(&self, ctx: CliContext<R>) -> Result<(), Error>
    where
        R: ExpensesRepository,
    {
        let expenses = ctx.repo.get_all(10)?;

        let table = Table::new(expenses)
            .with(Width::truncate(ctx.termsize.cols as usize))
            .to_string();
        println!("{table}");

        Ok(())
    }
}

#[derive(Args, Debug)]
pub struct DeleteExpense {
    uuid: String,
}

impl ProcessCommand for DeleteExpense {
    fn process<R>(&self, ctx: CliContext<R>) -> Result<(), Error>
    where
        R: ExpensesRepository,
    {
        // ensure we use minimally a uuid of 8 len to avoid deleting other expenses
        if self.uuid.len() < 8 {
            // FIX: there is probably a better way to write this
            println!("{}", AppError::InvalidParameter);
            return Err(AppError::InvalidParameter)?;
        }

        ctx.repo.delete(&format!("{}%", self.uuid))?;

        Ok(())
    }
}
