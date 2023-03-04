use chrono::NaiveDate;
use thiserror::Error;

#[derive(Debug)]
pub struct StatementEntry {
    date: NaiveDate,
    description: String,
    amount: f64,
    balance: f64,
}

impl StatementEntry {
    pub fn new(date: NaiveDate, description: String, amount: f64, balance: f64) -> StatementEntry {
        return StatementEntry { date, description, amount, balance }
    }
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Invalid data: {0}")]
    InvalidData(String)
}