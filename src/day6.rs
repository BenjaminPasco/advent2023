
use crate::utils::fetch;
use regex::{Regex, Captures};

struct Day6Data<'a> {
    url: &'a str,
    session_cookie: &'a str,
}

fn get_day_6_data () -> Day6Data<'static> {
    return Day6Data {
        url: "https://adventofcode.com/2023/day/6/input",
        session_cookie: "53616c7465645f5f6c9d212437e99ebeb3ac50c2520d2a271b50085457968466beab41babc3f48f87e996f2412054c8ee43c282f092ac2aec42e0687b7539ea5"
    }
}

pub async fn solve_part_one () {
    println!("part 1");

    let data = get_day_6_data();
    let request = fetch(data.url, data.session_cookie).await;

    match request {
        Err(_) => {
            println!("Bad Request");
        }
        Ok(body) => {

            let data = body.text().await.unwrap();
            let mut product = 1;

            let re = Regex::new(r".*:\s*([\d]*)\s*([\d]*)\s*([\d]*)\s*([\d]*)").unwrap();
            
            let captures: Vec<Captures> = data.lines().map(|line| re.captures(&line).unwrap()).collect();

            let vectorized_data: Vec<[i32; 4]> = captures.iter().map(|c| {
                let (_, extracts): (_, [&str; 4]) = c.extract();
                return extracts.map(|s| s.parse().unwrap());
            }).collect();

            let times = vectorized_data[0];
            let distances = vectorized_data[1];

            for race in 0..4 {
                let mut count = 0;
                for acceleration in 1..times[race] {
                    let distance_travelled = acceleration * (times[race] - acceleration);
                    if distance_travelled > distances[race] {
                        count = count + 1
                    }
                }
                product = product * count
            }

            println!("Product: {}", product)

        }
    }
}


pub async fn solve_part_two () {
    println!("part 2");

    let data = get_day_6_data();
    let request = fetch(data.url, data.session_cookie).await;

    match request {
        Err(_) => {
            println!("Bad Request");
        }
        Ok(body) => {

            let data = body.text().await.unwrap();

            let re = Regex::new(r".*:\s*([\d]*)\s*([\d]*)\s*([\d]*)\s*([\d]*)").unwrap();
            
            let captures: Vec<Captures> = data.lines().map(|line| re.captures(&line).unwrap()).collect();

            let vectorized_data: Vec<i64> = captures.iter().map(|c| {
                let (_, extracts): (_, [&str; 4]) = c.extract();
                let s = extracts.iter().cloned().collect::<String>();
                println!("s: {}", s);
                return s.parse().unwrap()
            }).collect();

            let time = vectorized_data[0];
            let distance = vectorized_data[1];

  
            let mut count = 0;
            for acceleration in 1..time {
                let distance_travelled = acceleration * (time - acceleration);
                if distance_travelled > distance {
                    count = count + 1
                }
            }
            println!("count: {}", count)

        }
    }
}