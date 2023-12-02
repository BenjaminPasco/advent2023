use crate::utils::fetch;

struct Day1Data<'a> {
    url: &'a str,
    session_cookie: &'a str,
}

fn get_day_1_data () -> Day1Data<'static> {
    return Day1Data {
        url: "https://adventofcode.com/2023/day/1/input",
        session_cookie: "53616c7465645f5f6c9d212437e99ebeb3ac50c2520d2a271b50085457968466beab41babc3f48f87e996f2412054c8ee43c282f092ac2aec42e0687b7539ea5"
    }
}

fn get_value(found_number: String) -> u32 {
    let numeric_value = found_number.parse::<u32>();
    match numeric_value {
        Ok(value) => {
            return value;
        }
        Err(_) => {
            println!("Could not convert this number");
            return 0
        }
    }
}

trait OptionCharExt {
    fn get_char(&self) -> char;
}

impl OptionCharExt for Option<char> {
    fn get_char(&self) -> char {
        match self {
            Some(char) => {
                return *char; 
            }
            None => {
                println!("No number on this line");
                return ' ';
            }
        }
    }
}

pub async fn solve_part_one () {
    println!("part 1");

    let data = get_day_1_data();
    let request = fetch(data.url, data.session_cookie).await;
    
    match request {
        Ok(response) => {
            let text = response.text().await;

            match text {
                Ok(content) => {
                    let str_ref: &str = &content;

                    let mut sum: u32 = 0;

                    for line in str_ref.lines() {
                        let first_number = line.chars().find(|c| c.is_numeric()).get_char();
                        let last_number = line.chars().rev().find(|c| c.is_numeric()).get_char();
                        let whole_number = first_number.to_string() + &last_number.to_string();
                        sum = sum + get_value(whole_number);
                    }

                    println!("Total sum: {}", sum)
                }
                Err(_) => {
                    println!("Error: couldnt parse")
                }
                
            }

        }
        Err(_) => {
            println!("Error: request failed")
        }
    }
}

fn get_number<I>(chars: I, numbers: [&str; 9]) -> String
where
    I: Iterator<Item = char>,
{
    let mut a: String = "".to_string();

    for char in chars {
        if char.is_numeric() {
            return char.to_string();
        }
        a = format!("{}{}", char, a);

        let position = numbers.iter().position(|&x| a.starts_with(x));
        if let Some(number) = position {
            return format!("{}", number+1);
        }
    }
    ' '.to_string()
}

pub async fn solve_part_two () {
    println!("part 2");

    let data = get_day_1_data();
    let request = fetch(data.url, data.session_cookie).await;

    match request {
        Ok(response) => {
            let text = response.text().await;
        
            match text {
                Ok(content) => {
                    let str_ref: &str = &content;

                    let mut sum: u32 = 0;

                    for line in str_ref.lines() {

                        let reversed_numbers = [
                            "eno",
                            "owt",
                            "eerht",
                            "ruof",
                            "evif",
                            "xis",
                            "neves",
                            "thgie",
                            "enin"
                        ];
                        let first_number = get_number(line.chars(), reversed_numbers);
                        let numbers = [
                            "one",
                            "two",
                            "three",
                            "four",
                            "five",
                            "six",
                            "seven",
                            "eight",
                            "nine"
                        ];
                        let a = line.chars().rev();
                        let last_number = get_number(a, numbers);

                        let whole_number = first_number + &last_number.to_string();
                        sum = sum + get_value(whole_number);
                    }

                    println!("Total sum: {}", sum)
                }
                Err(_) => {
                    println!("Error: couldnt parse")
                }
                
            }
        }
        Err(_) => {
            println!("Error: request failed");
        }
        
    }
}