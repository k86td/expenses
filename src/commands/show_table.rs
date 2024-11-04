use std::ops::RangeBounds;

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

use crate::{cli::ProcessCommand, models::Expense, styling::AsciiStyling};

// type ExpenseFilter = Option<Box<dyn Fn(&Expense) -> bool>>;

#[derive(Debug, Args)]
pub struct ShowTableExpense {
    // filter: ExpenseFilter,
}

impl ProcessCommand for ShowTableExpense {
    fn process<R>(&self, ctx: crate::models::CliContext<R>) -> Result<(), crate::Error>
    where
        R: crate::repository::ExpensesRepository,
    {
        let mut expenses = ctx.repo.get_all(25)?;

        // if let Some(filter) = &self.filter {
        //     expenses = expenses.into_iter().filter(filter).collect();
        // }

        let underline_style = Color::new(AsciiStyling::Underline, AsciiStyling::Reset);
        let mut display_table = Table::new(expenses);
        display_table
            .with(Style::blank())
            .with(Colorization::rows([Color::BG_BLACK, Color::BG_WHITE]))
            .with(Colorization::exact([underline_style], Rows::first()));

        for row in (2..display_table.count_rows()).step_by(2) {
            display_table.modify(Rows::single(row), BorderColor::new().left(Color::BG_BLACK));
        }

        for row in (1..display_table.count_rows()).step_by(2) {
            display_table.modify(Rows::single(row), BorderColor::new().left(Color::BG_WHITE));
        }

        println!("{}", display_table);

        Ok(())
    }
}
