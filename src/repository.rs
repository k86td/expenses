use crate::models::Expense;

pub trait ExpensesRepository<E> {
    /// Creates a new expenses.
    fn create(&self, expense: Expense) -> Result<usize, E>;

    /// Update an expense. The uuid of the Expense will be used
    /// to track the targetted Expense.
    fn update(&self, expense: Expense) -> Result<usize, E>;

    /// Delete an expense using its uuid.
    fn delete(&self, uuid: &str) -> Result<usize, E>;

    /// Get all expenses using a limit on the number of rows.
    fn get_all(&self, limit: u32) -> Result<Vec<Expense>, E>;
}
