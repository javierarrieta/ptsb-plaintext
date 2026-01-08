use crate::data::{AppError, StatementLine, TransactionEnum};
use calamine::{open_workbook, Data, Reader, Xls, XlsError};
use chrono::NaiveDate;

const MS_DAY: i64 = 1000 * 60 * 60 * 24;

impl From<XlsError> for AppError {
    fn from(value: XlsError) -> Self {
        AppError::InvalidData(value.to_string())
    }
}

fn parse_date(d: &Data) -> Option<NaiveDate> {
    match d {
        Data::DateTime(date) => {
            let i: i64 = date.as_f64() as i64;
            let epoch_days: i32 = (i / MS_DAY) as i32;
            NaiveDate::from_epoch_days(epoch_days)
        }
        Data::String(date) => NaiveDate::parse_from_str(date, "%d/%m/%Y").ok(),

        _ => None,
    }
}

fn parse_row(d: &[Data]) -> Option<StatementLine> {
    let maybe_date = d.get(0).and_then(parse_date);
    let maybe_description = match d.get(2) {
        Some(Data::String(s)) => Some(s),
        _ => None,
    };
    let maybe_amount = match (d.get(3), d.get(4)) {
        (Some(Data::Float(i)), _) => Some(i.to_owned()),
        (_, Some(Data::Float(out))) => Some(out.to_owned()),
        _ => None,
    };
    let maybe_balance = match d.get(6) {
        Some(Data::Float(balance)) => Some(balance.to_owned()),
        _ => None,
    };

    match (maybe_date, maybe_description, maybe_amount, maybe_balance) {
        (Some(date), Some(desc), Some(amount), Some(balance)) => Some(StatementLine::new(
            date,
            desc.to_string(),
            amount,
            guess_tx_type(desc),
            balance,
        )),
        _ => None,
    }
}

fn guess_tx_type(description: &str) -> Option<TransactionEnum> {
    let tx_descriptor = description.split_ascii_whitespace().next();
    match tx_descriptor {
        Some("DD") => Some(TransactionEnum::DirectDebit),
        Some("ICT") => Some(TransactionEnum::Transfer),
        Some("CT") => Some(TransactionEnum::DirectDeposit),
        Some("POS") | Some("TKN") | Some("CNC") => Some(TransactionEnum::POS),
        _ => None,
    }
}

pub fn parse_file(file_path: &str) -> Result<Vec<StatementLine>, AppError> {
    let mut workbook: Xls<_> = open_workbook(file_path)?;

    let range = workbook
        .worksheets()
        .first()
        .map(|r| r.1.to_owned())
        .ok_or(AppError::InvalidData("No sheets present".to_string()))?;

    Ok(range.rows().filter_map(parse_row).collect())
}
