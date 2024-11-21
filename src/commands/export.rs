use crate::{
    cli::ProcessCommand,
    models::{self, Expense},
};

use clap::Args;
use mlua::prelude::*;

#[derive(Args, Debug)]
pub struct InlineExportExpenses {
    script: String,
}

impl ProcessCommand for InlineExportExpenses {
    fn process<R>(&self, ctx: crate::models::CliContext<R>) -> Result<(), crate::Error>
    where
        R: crate::repository::ExpensesRepository,
    {
        let lua = Lua::new();
        let expenses = expense_to_table(&lua, ctx.repo.get_all(1000)?)?;

        lua.globals().set("expenses", expenses)?;

        lua.load(&self.script).exec()?;

        Ok(())
    }
}

fn expense_to_table(lua: &Lua, expenses: Vec<Expense>) -> Result<mlua::Table, mlua::Error> {
    let index_table = lua.create_table()?;

    for (index, value) in expenses.iter().enumerate() {
        let inner_table = lua.create_table()?;
        inner_table.set("uuid", value.uuid.clone())?;
        inner_table.set("created", value.created.to_string())?;
        inner_table.set("modified", value.modified.to_string())?;
        inner_table.set("data", value.data.to_string())?;

        index_table.set(index, inner_table)?;
    }

    Ok(index_table)
}
