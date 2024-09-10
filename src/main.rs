use polars::prelude::*;
use std::fs::File;

pub fn calculate() -> Result<DataFrame, PolarsError> {
    let file = File::open("data/sample.csv")?;
    let df = CsvReader::new(file).finish()?.lazy();

    let df_transformed = df
        .select(&[col("foo"), col("bar")])
        .group_by_stable([col("foo")])
        .agg([col("bar").min().alias("min_bar")])
        .collect()?;
    Ok(df_transformed)
}

fn main() {
    let df = calculate().unwrap();
    println!("{}", df);
}
