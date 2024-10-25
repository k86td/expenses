use crate::{models::Expense, Result};

pub trait ExpensesRepository {
    /// Creates a new expenses.
    fn create(&self, expense: Expense) -> Result<usize>;

    /// Update an expense. The uuid of the Expense will be used
    /// to track the targetted Expense.
    fn update(&self, expense: Expense) -> Result<usize>;

    /// Delete an expense using its uuid.
    fn delete(&self, uuid: &str) -> Result<usize>;

    /// Get all expenses using a limit on the number of rows.
    fn get_all(&self, limit: u32) -> Result<Vec<Expense>>;
}
