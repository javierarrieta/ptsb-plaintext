use chrono::NaiveDate;
use serde::{Serialize, Serializer};
use thiserror::Error;

#[derive(Debug, Serialize, PartialOrd, PartialEq)]
pub struct StatementLine {
    #[serde(serialize_with = "serialize_date")]
    date: NaiveDate,
    description: String,
    amount: f64,
    tx_type: Option<TransactionEnum>,
    balance: f64,
}

fn serialize_date<S>(x: &NaiveDate, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(&x.to_string())
}

impl StatementLine {
    pub fn new(
        date: NaiveDate,
        description: String,
        amount: f64,
        tx_type: Option<TransactionEnum>,
        balance: f64,
    ) -> StatementLine {
        return StatementLine {
            date,
            description,
            amount,
            tx_type,
            balance,
        };
    }
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Invalid data: {0}")]
    InvalidData(String),
    #[error("IO Error: {0}")]
    IOError(String),
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),
}

impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        AppError::IOError(value.to_string())
    }
}

// Define the TransactionEnum enum
#[derive(Debug, Serialize, PartialEq, Eq, PartialOrd)]
pub enum TransactionEnum {
    Credit,
    Debit,
    Int,
    Div,
    Fee,
    ServiceCharge,
    Deposit,
    ATM,
    POS,
    Transfer,
    Check,
    Payment,
    Cash,
    DirectDeposit,
    DirectDebit,
    RepeatPayment,
    Other,
}
