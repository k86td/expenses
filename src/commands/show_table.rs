use std::{fmt::Debug, ops::RangeBounds};

use clap::Args;
use tabled::{
    settings::{
        object::{Column, Columns, Object, Rows},
        style::BorderColor,
        themes::Colorization,
        Border, Color, Style, Theme,
    },
    Table,
};

use crate::{
    cli::ProcessCommand,
    models::Expense,
    styling::{new_table, AsciiStyling},
};

#[derive(Args)]
pub struct ShowTableExpense;

impl Debug for ShowTableExpense {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ShowTableExpense")
    }
}

impl ProcessCommand for ShowTableExpense {
    fn process<R>(&self, ctx: crate::models::CliContext<R>) -> Result<(), crate::Error>
    where
        R: crate::repository::ExpensesRepository,
    {
        let expenses = ctx.repo.get_all(25)?;

        let display_table = new_table(expenses);

        println!("{}", display_table);

        Ok(())
    }
}
