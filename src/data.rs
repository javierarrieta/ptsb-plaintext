use chrono::NaiveDate;
use thiserror::Error;
use serde::{Serialize, Serializer};

#[derive(Debug, Serialize)]
pub struct StatementEntry {
    #[serde(serialize_with="serialize_date")]
    date: NaiveDate,
    description: String,
    amount: f64,
    balance: f64,
}

fn serialize_date<S>(x: &NaiveDate, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
{
    s.serialize_str(&x.to_string())
}

impl StatementEntry {
    pub fn new(date: NaiveDate, description: String, amount: f64, balance: f64) -> StatementEntry {
        return StatementEntry { date, description, amount, balance }
    }
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Invalid data: {0}")]
    InvalidData(String),
    #[error("IO Error: {0}")]
    IOError(String),
}

impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        AppError::IOError(value.to_string())
    }
}