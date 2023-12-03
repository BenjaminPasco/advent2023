use std::error::Error;

mod day1;
mod day2;
mod day3;
mod utils;

#[tokio::main]
async fn main ()  -> Result<(), Box<dyn Error>>{

    println!("Day 1");
    day1::solve_part_one().await;
    day1::solve_part_two().await;
    println!("Day 2");
    day2::solve_part_one().await;
    day2::solve_part_two().await;
    println!("Day 3");
    day3::solve_part_one().await;
    day3::solve_part_two().await;
    Ok(())
}

