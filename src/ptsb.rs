use calamine::{DataType, open_workbook, Reader, Xls, XlsError};
use calamine::DataType::{DateTime, Float, String as DString};
use chrono::NaiveDate;
use crate::data::{StatementEntry, AppError};

const MS_DAY: i64 = 1000 * 60 * 60 * 24;

impl From<calamine::Error> for AppError {
    fn from(value: calamine::Error) -> Self {
        return AppError::InvalidData(value.to_string())
    }
}

impl From<calamine::XlsError> for AppError {
    fn from(value: XlsError) -> Self {
        return AppError::InvalidData(value.to_string())
    }
}

fn parse_date(d: &DataType) -> Option<NaiveDate> {
    match d {
        DateTime(date) => {
            let i: i64 = *date as i64;
            let epoch_days: i32 = (i / MS_DAY) as i32;
            return NaiveDate::from_epoch_days(epoch_days);
        }
        DString(date) => {
            return NaiveDate::parse_from_str(date, "%d/%m/%Y").ok();
        }
        _ => return None
    }
}

fn parse_row(d: &[DataType]) -> Option<StatementEntry> {
    let maybe_date = d.get(0).and_then(parse_date);
    let maybe_description = match d.get(2) {
        Some(DString(s)) => Some(s),
        _ => None,
    };
    let maybe_amount = match (d.get(3), d.get(4)) {
        (Some(Float(i)), _) => Some(i.to_owned()),
        (_, Some(Float(out))) => Some(out.to_owned()),
        _ => None,
    };
    let maybe_balance = match d.get(6) {
        Some(Float(balance)) => Some(balance.to_owned()),
        _ => None
    };

    return match (maybe_date, maybe_description, maybe_amount, maybe_balance) {
        (Some(date), Some(desc), Some(amount), Some(balance)) =>
            Some(StatementEntry::new(date, desc.to_string(), amount, balance)),
        _ => None
    }
}

pub fn parse_file(file_path: &str) -> Result<Vec<StatementEntry>, AppError> {

    let mut workbook: Xls<_> = open_workbook(file_path)?;

    let range = workbook.worksheets().first()
        .map(|r| r.1.to_owned())
        .ok_or(AppError::InvalidData("No sheets present".to_string()))?;

    return  Ok(range.rows().filter_map(parse_row).collect());
}