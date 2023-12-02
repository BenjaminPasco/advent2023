use std::error::Error;

mod day1;
mod utils;

#[tokio::main]
async fn main ()  -> Result<(), Box<dyn Error>>{

    day1::solve_part_one().await;
    day1::solve_part_two().await;

    Ok(())
}

