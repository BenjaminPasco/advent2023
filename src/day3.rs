use crate::utils::fetch;

struct Day3Data<'a> {
    url: &'a str,
    session_cookie: &'a str,
}

fn get_day_3_data () -> Day3Data<'static> {
    return Day3Data {
        url: "https://adventofcode.com/2023/day/3/input",
        session_cookie: "53616c7465645f5f6c9d212437e99ebeb3ac50c2520d2a271b50085457968466beab41babc3f48f87e996f2412054c8ee43c282f092ac2aec42e0687b7539ea5"
    }
}

struct Number {
    value: i32,
    line: usize,
    start: usize,
    end: usize
}

struct Symbol {
    kind: char,
    line: usize,
    position: usize
}

struct Engine {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>
}

impl ToString for Number {
    fn to_string(&self) -> String {
        return format!("value: {}, line: {}, starts: {}, ends: {} \n", self.value, self.line, self.start, self.end);
    }
}

impl ToString for Symbol {
    fn to_string(&self) -> String {
        return format!("kind: {}, line: {}, position: {} \n", self.kind, self.line, self.position);
    }
}

impl ToString for Engine {
    fn to_string(&self) -> String {

        let mut text = String::new();
        text.push_str("Engine: \n");
        text.push_str("     Symbols: \n");
        self.symbols.iter().for_each(|symbol| text.push_str(&symbol.to_string()));
        self.numbers.iter().for_each(|number| text.push_str(&number.to_string()));
        return text;

    }
}

fn format_data(text: &str) -> Engine {

    let mut engine = Engine {
        numbers: vec![],
        symbols: vec![]
    }; 

    for (line_index, line) in text.lines().enumerate() {

        let mut start = 0;
        let mut value = String::new();

        for (char_index, character) in line.chars().enumerate(){

            if character.is_numeric() {
                value.push(character);

                if char_index == 0 {
                    start = char_index
                }
                else if !line.chars().nth(char_index - 1).unwrap().is_numeric() {
                    start = char_index;
                }

                let next_char = line.chars().nth(char_index + 1);
                let mut is_next_numeric = false;
                match next_char {
                    None => {
                        is_next_numeric = false
                    }
                    Some(char) => {
                        is_next_numeric = char.is_numeric()
                    }
                }

                if !is_next_numeric {
                    let number = Number {
                        start: start,
                        end: char_index,
                        line: line_index,
                        value: value.parse().unwrap()
                    };
                    engine.numbers.push(number);
                    start = 0;
                    value = String::new();

                }
            }
            else if character == '.' {

            }
            else {
                let symbol = Symbol {
                    line: line_index,
                    position: char_index,
                    kind: character
                };
                engine.symbols.push(symbol);

            }

        }
    }

    return engine
}

fn is_close (number: &Number, symbol: &Symbol) -> bool{
    
    let close_line = number.line + 1 >= symbol.line && number.line <= symbol.line + 1;
    
    let close_position = number.start <= symbol.position + 1 && symbol.position <= number.end + 1;

    return close_line && close_position
}

fn is_part (number: &Number, symbols: &Vec<Symbol>) -> bool {
    return symbols.iter().any(|symbol| is_close(number, symbol));
}

pub async fn solve_part_one () {
    println!("part 1");

    let data = get_day_3_data();
    let request = fetch(data.url, data.session_cookie).await;

    match request {
        Err(_) => {
            println!("Bad Request");
        }
        Ok(body) => {

            let data = body.text().await.unwrap();
            let engine = format_data(&data);

            let mut sum = 0;

            for number in engine.numbers {
                if is_part(&number, &engine.symbols) {
                    sum = sum + number.value;
                }
            }   

            println!("sum: {}", sum);
        }
        
    }
}

fn get_adjacent_numbers<'a> (symbol: &Symbol, numbers: &'a Vec<Number>) -> Vec<&'a Number> {
    
    let mut adjacent_numbers: Vec<&Number> = vec![];

    for number in numbers {
        if is_close(number, symbol) {
            adjacent_numbers.push(number)
        }
    }

    return adjacent_numbers;
}

pub async fn solve_part_two () {
    println!("part 2");

    let data = get_day_3_data();
    let request = fetch(data.url, data.session_cookie).await;

    match request {
        Err(_) => {
            println!("Bad Request");
        }
        Ok(body) => {

            let data = body.text().await.unwrap();
            let engine = format_data(&data);
            let mut sum = 0;

            for symbol in engine.symbols {
                if symbol.kind == '*' {
                    
                    let numbers = get_adjacent_numbers(&symbol, &engine.numbers);
                    if numbers.len() == 2 {
                        sum = sum + numbers[0].value * numbers[1].value
                    }

                }
            }   

            println!("sum: {}", sum);
        }
        
    }
}
