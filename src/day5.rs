
use crate::utils::fetch;
use regex::Regex;

struct Day5Data<'a> {
    url: &'a str,
    session_cookie: &'a str,
}

fn get_day_5_data () -> Day5Data<'static> {
    return Day5Data {
        url: "https://adventofcode.com/2023/day/5/input",
        session_cookie: "53616c7465645f5f6c9d212437e99ebeb3ac50c2520d2a271b50085457968466beab41babc3f48f87e996f2412054c8ee43c282f092ac2aec42e0687b7539ea5"
    }
}

struct Entry {
    destination_range: i64,
    source_range: i64,
    range_length: i64
}

fn string_to_map (raw: &str) -> Vec<Entry> {

    let mut entries: Vec<Entry> = vec![];

    for line in raw.lines() {

        let a: Vec<&str> = line.split_whitespace().collect();

        entries.push(Entry {
            destination_range: a[0].parse::<i64>().unwrap(),
            source_range: a[1].parse::<i64>().unwrap(),
            range_length: a[2].parse::<i64>().unwrap()
        })
    }

    return entries
}

fn get_seed_location (seed: i64, maps: &Vec<Vec<Entry>>) -> i64 {
    let mut location = seed;

    for map in maps {
        for entry in map {
            if location >= entry.source_range && location < entry.source_range + entry.range_length {
                let a = location - entry.source_range;
                let b = entry.destination_range + a;
                location = b;
                break;
            }
        }
    }

    return location;
}

pub async fn solve_part_one () {
    println!("part 1");

    let data = get_day_5_data();
    let request = fetch(data.url, data.session_cookie).await;

    match request {
        Err(_) => {
            println!("Bad Request");
        }
        Ok(body) => {

            let data = body.text().await.unwrap();
            let mut sum = 0;

            let re = Regex::new(r"\s([\d*|\s]*)\n.*:\n([\d*|\s]*)\n\n.*\n([\d*|\s]*)\n\n.*\n([\d*|\s]*)\n\n.*\n([\d*|\s]*)\n\n.*\n([\d*|\s]*)\n\n.*\n([\d*|\s]*)\n\n.*\n([\d*|\s]*)").unwrap();

            let captures = re.captures(&data).unwrap();
            let seeds: Vec<i64> = captures[1].split_whitespace().map(|s| s.parse::<i64>()).filter_map(Result::ok).collect();
            let raw_maps = vec![&captures[2], &captures[3], &captures[4], &captures[5], &captures[6], &captures[7], &captures[8]];

            let maps: Vec<Vec<Entry>> = raw_maps.iter().map(|&raw_map| string_to_map(raw_map)).collect();
            let mut min = 0;

            for seed in seeds {
                let seed_score = get_seed_location(seed, &maps);
                if min == 0 {
                    min = seed_score
                }
                else if seed_score < min {
                    min = seed_score
                }
            }

            println!("min: {}", min)

        }
    }

}

pub async fn solve_part_two () {
    println!("part 2");

    let data = get_day_5_data();
    let request = fetch(data.url, data.session_cookie).await;

    match request {
        Err(_) => {
            println!("Bad Request");
        }
        Ok(body) => {

            let data = body.text().await.unwrap();
            let mut sum = 0;

            let re = Regex::new(r"\s([\d*|\s]*)\n.*:\n([\d*|\s]*)\n\n.*\n([\d*|\s]*)\n\n.*\n([\d*|\s]*)\n\n.*\n([\d*|\s]*)\n\n.*\n([\d*|\s]*)\n\n.*\n([\d*|\s]*)\n\n.*\n([\d*|\s]*)").unwrap();

            let captures = re.captures(&data).unwrap();
            let seeds: Vec<i64> = captures[1].split_whitespace().map(|s| s.parse::<i64>()).filter_map(Result::ok).collect();
            let raw_maps = vec![
                &captures[2],
                &captures[3],
                &captures[4],
                &captures[5],
                &captures[6], 
                &captures[7], 
                &captures[8]
            ];

            let maps: Vec<Vec<Entry>> = raw_maps.iter().map(|&raw_map| string_to_map(raw_map)).collect();
            let mut min = 0;

            for (index, seed) in seeds.iter().enumerate() {

                if index % 2 == 0 {
                    println!("index: {}, seed: {}, range: {}", index, seed, seeds[index + 1]);
                    for i in 0..seeds[index + 1] {
                        let seed_score = get_seed_location(seed + i, &maps);
                        if min == 0 {
                            min = seed_score
                        }
                        else if seed_score < min {
                            min = seed_score;
                            println!("min updated: {}", min)
                        }
                    }
                    println!("min found: {}, for: {}", min, seed)   
                }
            }

            println!("min: {}", min)

        }
    }

}