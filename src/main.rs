use polars::prelude::*;
use std::fs::File;

pub fn calculate() -> Result<DataFrame, PolarsError> {
    let file = File::open("data/sample.csv")?;
    let df = CsvReader::new(file).finish()?.lazy().collect()?;
    Ok(df)
}

fn main() {
    let df = calculate().unwrap();
    println!("{}", df);
}
