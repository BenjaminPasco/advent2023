use crate::utils::fetch;

struct Day2Data<'a> {
    url: &'a str,
    session_cookie: &'a str,
}

fn get_day_2_data () -> Day2Data<'static> {
    return Day2Data {
        url: "https://adventofcode.com/2023/day/2/input",
        session_cookie: "53616c7465645f5f6c9d212437e99ebeb3ac50c2520d2a271b50085457968466beab41babc3f48f87e996f2412054c8ee43c282f092ac2aec42e0687b7539ea5"
    }
}

struct Colors {
    red: i32,
    blue: i32,
    green: i32
}

struct Game {
    id: i32,
    rounds: Vec<Colors>
}

fn format_raw_data (raw_data: &str) -> Vec<Game> {

    let mut data:Vec<Game> = vec![];
    
    for line in raw_data.lines() {
        
        let mut game_data = Game {
            id: 0,
            rounds: vec![]
        };
        let parts: Vec<&str> = line.split(':').collect();
        let id = parts[0].strip_prefix("Game ");
        match id {
            Some(id)=> {
                game_data.id = id.parse::<i32>().unwrap();
            }
            None => {}
        }
        let games: Vec<&str> = parts[1].split(';').collect();
        
        for game in games {
            let mut round_data = Colors {
                red: 0,
                blue: 0,
                green: 0
            };
            let colors: Vec<&str> = game.split(',').collect();

            for color in colors {
                let data: Vec<&str> = color.trim().split_whitespace().collect();

                if data[1] == "red" {
                    round_data.red = data[0].parse::<i32>().unwrap();
                }
                else if data[1] == "blue" {
                    round_data.blue = data[0].parse::<i32>().unwrap();
                }
                else if data[1] == "green" {
                    round_data.green = data[0].parse::<i32>().unwrap();
                }
            }

            game_data.rounds.push(round_data)

        }

        data.push(game_data);

    }
    return data;
}

pub async fn solve_part_one () {
    println!("part 1");

    let data = get_day_2_data();
    let red = 12;
    let green = 13;
    let blue = 14; 
    let request = fetch(data.url, data.session_cookie).await;

    match request {
        Ok(body) => {

            match body.text().await {
                Ok(raw_data) => {
                    
                    let mut sum = 0;

                    let games = format_raw_data(&raw_data);

                    for game in games {

                        let test = game.rounds.iter().any(|round| round.red > red || round.green > green || round.blue > blue);

                        if test == false {
                            sum = sum + game.id
                        } 
                    
                    }

                    println!("Sum ids of game possible: {}", sum)

                }
                Err(_) => {
                    println!("Could not parse")
                }
            }
        }
        Err(_) => {
            println!("Bad request")
        }
        
    }
}

pub async fn solve_part_two () {
    println!("part 2");

    let data = get_day_2_data();
    let request = fetch(data.url, data.session_cookie).await;

    match request {
        Ok(body) => {

            match body.text().await {
                Ok(raw_data) => {
                    
                    let mut sum = 0;

                    let games = format_raw_data(&raw_data);

                    for game in games {

                        let mut red_max = 0;
                        let mut blue_max = 0;
                        let mut green_max = 0;
                        
                        for round in game.rounds {
                            if round.red > red_max {
                                red_max = round.red
                            }
                            if round.blue > blue_max {
                                blue_max = round.blue
                            }
                            if round.green > green_max {
                                green_max = round.green
                            }
                        }

                        sum = sum + red_max * blue_max * green_max;
                    
                    }

                    println!("Sum ids of game possible: {}", sum)

                }
                Err(_) => {
                    println!("Could not parse")
                }
            }
        }
        Err(_) => {
            println!("Bad request")
        }
        
    }
}