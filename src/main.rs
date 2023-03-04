mod data;
mod ptsb;

use std::env;
use std::string::String;
use crate::data::AppError;

fn main() -> Result<(), AppError> {
    let arguments: Vec<String> = env::args().collect();
    let fallback_path = env::var("PTSB_XLS_FILE").ok().unwrap_or("".to_string());
    let f_from_args = arguments.get(1);
    let f = f_from_args
        .unwrap_or(&fallback_path);

    let rows = ptsb::parse_file(f)?;

    println!("{:?}", rows);

    return Ok(())
}
