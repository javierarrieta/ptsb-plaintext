use std::env;
use std::string::String;
use calamine::{DataType, open_workbook, Reader, Xls};
use calamine::DataType::{DateTime, Float, String as DString};
use chrono::{NaiveDate, NaiveDateTime};

#[derive(Debug)]
struct StatementEntry {
    date: NaiveDate,
    description: String,
    amount: f64,
    balance: f64,
}

impl StatementEntry {
    fn new(date: NaiveDate, description: String, amount: f64, balance: f64) -> StatementEntry {
        return StatementEntry { date, description, amount, balance }
    }
}

fn parse_date(d: &DataType) -> Option<NaiveDate> {
    match d {
        DateTime(date) => {
            let i: i64 = *date as i64;
            return NaiveDateTime::from_timestamp_millis(i).map(|ndt| ndt.date());
        }
        DString(date) => {
            return NaiveDate::parse_from_str(date, "%d/%m/%Y").ok();
        }
        _ => return None
    }
}

fn parse_row(d: &[DataType]) -> Option<StatementEntry> {
    let maybe_date = d.get(0).and_then(parse_date);
    if maybe_date.is_some() { println!("{:?}", d); }
    let maybe_description = match d.get(2) {
        Some(DString(s)) => Some(s),
        _ => None,
    };
    let maybe_amount = match (d.get(3), d.get(4)) {
        (Some(Float(i)), _) => Some(i.to_owned()),
        (_, Some(Float(out))) => Some(-out),
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

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let fallback_path = env::var("PTSB_XLS_FILE").ok().unwrap_or("".to_string());
    let f_from_args = arguments.get(1);
    let f = f_from_args
        .unwrap_or(&fallback_path);

    let mut workbook: Xls<_> = open_workbook(f).expect(format!("Cannot open file: {}", f).as_str());

    if let Some(Ok(range)) = workbook.worksheet_range("ViewTxns_XLS") {
        println!("{:?}", range.get_size());

        let rows: Vec<StatementEntry> = range.rows().filter_map(parse_row).collect();

        println!("{:?}", rows);
    }
}
