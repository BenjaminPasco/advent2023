use std::error::Error;

mod day1;
mod day2;
mod utils;

#[tokio::main]
async fn main ()  -> Result<(), Box<dyn Error>>{

    println!("Day 1");
    day1::solve_part_one().await;
    day1::solve_part_two().await;
    println!("Day 2");
    day2::solve_part_one().await;
    day2::solve_part_two().await;
    Ok(())
}

