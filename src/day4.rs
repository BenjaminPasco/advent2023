use crate::utils::fetch;
use regex::Regex;

struct Day4Data<'a> {
    url: &'a str,
    session_cookie: &'a str,
}

fn get_day_4_data () -> Day4Data<'static> {
    return Day4Data {
        url: "https://adventofcode.com/2023/day/4/input",
        session_cookie: "53616c7465645f5f6c9d212437e99ebeb3ac50c2520d2a271b50085457968466beab41babc3f48f87e996f2412054c8ee43c282f092ac2aec42e0687b7539ea5"
    }
}

struct Draw {
    id: i32,
    card_numbers: Vec<i32>,
    draw_numbers: Vec<i32>
}

fn format_data (text: &str) -> Vec<Draw> {

    let mut data: Vec<Draw> = vec![];

    for line in text.lines() {
        
        let mut draw = Draw {
            id: 0,
            card_numbers: vec![],
            draw_numbers: vec![]
        };
        let re = Regex::new(r"Card\s*(\d*):\s*([\d*\s*]*)\s\|\s([\d*\s*]*)").unwrap();

        for capture in re.captures_iter(line) {

            draw.id = capture[1].parse().unwrap();
            let card_numbers: Result<Vec<i32>, _> = capture[2].split_whitespace().map(|n| n.parse()).collect();
            draw.card_numbers = card_numbers.unwrap();
            let draw_numbers: Result<Vec<i32>, _> = capture[3].split_whitespace().map(|n| n.parse()).collect();
            draw.draw_numbers = draw_numbers.unwrap();

        }

        data.push(draw)

    }

    return data

}

pub async fn solve_part_one () {
    println!("part 1");

    let data = get_day_4_data();
    let request = fetch(data.url, data.session_cookie).await;

    match request {
        Err(_) => {
            println!("Bad Request");
        }
        Ok(body) => {

            let data = body.text().await.unwrap();
            let draws = format_data(&data);
            let mut sum = 0;

            draws.iter().for_each(|d| {
                let count = d.draw_numbers.iter().filter(|&dn| {
                    d.card_numbers.iter().any(|cn| dn == cn)
                }).count();
                if count == 0 {
                    return;
                }
                sum = sum + 2u32.pow(count as u32 - 1);
                }
            );
            println!("sum: {}",sum)
        }
    }
}

fn get_count(draw: &Draw) -> usize {
    return draw.draw_numbers
    .iter()
    .filter(|&dn| {
        draw.card_numbers
        .iter()
        .any(|cn| dn == cn)
    }).count();
}

fn get_value(index: usize, draws: &Vec<Draw>) -> u32 {
    if index > draws.len() {
        return 0
    }
    let count = get_count(&draws[index]);
    if count == 0 {
        return 1
    }
    else {
        let mut value = 0;
        for i in 0..count {
            value = value + get_value(index + 1 + i, draws)
        }
        return 1 + value
    }
}

pub async fn solve_part_two () {
    println!("part 2");

    let data = get_day_4_data();
    let request = fetch(data.url, data.session_cookie).await;

    match request {
        Err(_) => {
            println!("Bad Request");
        }
        Ok(body) => {

            let data = body.text().await.unwrap();
            let draws = format_data(&data);
            let mut sum = 0;


            for (index, _) in draws.iter().enumerate() {
                sum = sum + get_value(index, &draws)
            }

            println!("sum: {}", sum)
        }
    }
}