use crate::models::Expense;

pub trait ExpensesRepository {
    type E;

    /// Creates a new expenses.
    fn create(&self, expense: Expense) -> Result<usize, Self::E>;

    /// Update an expense. The uuid of the Expense will be used
    /// to track the targetted Expense.
    fn update(&self, expense: Expense) -> Result<usize, Self::E>;

    /// Delete an expense using its uuid.
    fn delete(&self, uuid: &str) -> Result<usize, Self::E>;

    /// Get all expenses using a limit on the number of rows.
    fn get_all(&self, limit: u32) -> Result<Vec<Expense>, Self::E>;
}
