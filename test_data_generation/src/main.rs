
use fake::{Dummy, Fake, Faker};
use fake::locales::{EN};
use chrono::Utc;
use fake::faker::chrono::raw::*;
use std::io::stdout;
use serde::Serialize;
use std::error::Error;

#[derive(Debug, Dummy, Serialize)]
pub struct DummyCSVForDataCleaning {
    #[dummy(faker = "100..200")]
    transaction_id: usize,
    cust_id: u64,
    #[dummy(faker = "Date(EN)")]
    tran_date: String,
    prod_subcat_code: u8,
    prod_cat_code: u8,
    Qty: i64,
    Rate: i64,
    Tax: f64,
    total_amt: f64,
    Store_type: String 
}


fn generate_date() -> String {
    /// Generate a random date centred around today, in the format %Y-%m-%d
    /// # Arguments
    /// None
    /// 
    /// # Examples
    /// 
    /// generate_date() -> 2024-07-17
    /// 
    let start_dt: chrono::DateTime<Utc> = DateTimeBefore(EN, Utc::now()).fake();
    let end_dt: chrono::DateTime<Utc> = DateTimeAfter(EN, Utc::now()).fake();

    let between: chrono::DateTime<Utc> = DateTimeBetween(EN, start_dt, end_dt).fake();
    return format!("{}", between.format("%Y-%m-%d"));
}

fn generate_and_serialize()-> Result<(), Box<dyn Error>>{

    let mut wtr = csv::Writer::from_writer(stdout());
    
    for i in 1..1000{
        let f: DummyCSVForDataCleaning = Faker.fake();    
        wtr.serialize(f)?;
    }
    
    wtr.flush()?;
    Ok(())
    
}

fn main(){
    if let Err(e) = generate_and_serialize(){
        eprintln!("{}", e);
    }
}