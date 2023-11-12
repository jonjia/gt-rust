use anyhow::Result;
use polars::prelude::*;
use queryer::query;
use std::io::Cursor;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let url = "https://raw.githubusercontent.com/owid/covid-19-data/master/public/data/latest/owid-covid-latest.csv";
    // let data = reqwest::get(url).await?.text().await?;

    // // 使用 polars 直接请求
    // let df = CsvReader::new(Cursor::new(data))
    //     .infer_schema(Some(16))
    //     .finish()?;

    // println!(
    //     "{:?}",
    //     df.select((
    //         "location",
    //         "total_cases",
    //         "new_cases",
    //         "total_deaths",
    //         "new_deaths"
    //     ))
    // );

    let sql = format!(
        "SELECT location name, total_cases, new_cases, total_deaths, new_deaths \
      FROM {} where new_deaths >= 0 ORDER BY new_cases DESC",
        url
    );

    let df = query(sql).await?;
    println!("{:?}", df);
    Ok(())
}
