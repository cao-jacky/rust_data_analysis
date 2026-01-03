use polars::prelude::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let df_csv = CsvReadOptions::default()
        .with_has_header(true)
        .with_parse_options(CsvParseOptions::default().with_try_parse_dates(true))
        .try_into_reader_with_file_path(Some("data/Iris.csv".into()))?
        .finish()?;

    println!("{df_csv}");

    let result = df_csv
        .clone()
        .lazy()
        .select([
            col("SepalLengthCm").mean().alias("SepalLengthCmMean"),
        ])
        .collect()?;
    println!("{result}");

    // let df = CsvReader::from_path("../data/Iris.csv")?
    //     .has_header(true)
    //     .infer_schema(None)
    //     .finish()?;

    // let stats = df
    //     .lazy()
    //     .select([
    //         cols("*").mean().alias("mean"),
    //         cols("*").std(1).alias("stddev"),
    //     ])
    //     .collect()?;

    // println!("{stats}");
    Ok(())
}
