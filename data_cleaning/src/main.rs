use tokio;
use datafusion::prelude::*;

#[tokio::main]
async fn main() -> datafusion::error::Result<()>{
    let csv_path = "example_2.csv";
    let n = 10;

    let ctx = SessionContext::new();
    ctx.register_csv("csv_to_read", &csv_path,CsvReadOptions::new()).await?;
    let sql_query = format!("SELECT * FROM csv_to_read LIMIT {n}");
    let df = ctx.sql(&(sql_query)).await?;

    df.show().await?;
    
    Ok(())
}
