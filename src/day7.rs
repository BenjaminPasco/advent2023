
use crate::utils::fetch;
use regex::{Regex, Captures};

struct Day7Data<'a> {
    url: &'a str,
    session_cookie: &'a str,
}

fn get_day_7_data () -> Day7Data<'static> {
    return Day7Data {
        url: "https://adventofcode.com/2023/day/7/input",
        session_cookie: "53616c7465645f5f6c9d212437e99ebeb3ac50c2520d2a271b50085457968466beab41babc3f48f87e996f2412054c8ee43c282f092ac2aec42e0687b7539ea5"
    }
}

struct Hand<'a> {
    bid: u32,
    cards: &'a str
}

fn value_of_card (card: &char) -> u32 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        _ => 0
    }
}

fn value_of_hand (hand: &Hand) -> (u32, [u32; 5]) {
    let mut hand_type = 0;
    let mut card_values: [u32; 5] = [0; 5];
    let mut first: u32 = 0;
    let mut second: u32 = 0;
    for (index ,char) in hand.cards.chars().enumerate() {
        let value = value_of_card(&char);
        card_values[index] = value;
        let count: u32 = card_values.iter().filter(|&&v|  v == value).count().try_into().unwrap();
        if count > first {
            
            first = count;

        }
        else if count > second {
            second = count;
        }
        
        
    }
    
    
    match (first, second) {
        (1, 1) => hand_type = 0,
        (2, 1) => hand_type = 1,
        (2, 2) => hand_type = 2,
        (3, 1) => hand_type = 3,
        (3, 2) => hand_type = 4,
        (4, 1) => hand_type = 5,
        (5, 0) => hand_type = 6,
        (_, _) => ()
    }
    
    
    return (hand_type, card_values)
}

fn compare_value_cards (a: [u32; 5], b: [u32; 5]) -> std::cmp::Ordering {
    for index in 0..5 {
        if a[index] < b[index] {
            return std::cmp::Ordering::Less
        }
        else if a[index] > b[index] {
            return std::cmp::Ordering::Greater
        }
    }
    return std::cmp::Ordering::Equal
}

fn compare_hands (a: &Hand, b: &Hand) -> std::cmp::Ordering {
    let (t_a, c_a) = value_of_hand(a);
    let (t_b, c_b) = value_of_hand(b);
    if t_a < t_b {
        return std::cmp::Ordering::Less;
    }
    else if t_a > t_b {
        return std::cmp::Ordering::Greater;
    }
    else {
        return compare_value_cards(c_a, c_b)
    }
}

pub async fn solve_part_one () {
    println!("part 1");

    let data = get_day_7_data();
    let request = fetch(data.url, data.session_cookie).await;

    match request {
        Err(_) => {
            println!("Bad Request");
        }
        Ok(body) => {

            let data = body.text().await.unwrap();
            let mut hands: Vec<Hand> = vec![];

            data.lines().for_each(|line| {
                let line_data: Vec<&str> = line.split_whitespace().collect();
                let hand = Hand {
                    cards: line_data[0],
                    bid: line_data[1].parse().unwrap()
                };
                hands.push(hand);
            });

            hands.sort_by(|a, b| compare_hands(a, b));

            

            let mut sum: u32 = 0;

            for (index, bid) in hands.iter().map(|hand| hand.bid ).enumerate() {
                sum = sum + bid * (index as u32 + 1);
                println!("cards: {}, bid: {}", hands[index as usize].cards, bid)
            }
            println!("sum: {}", sum)

        }
    }
}

fn value_of_card2 (card: &char) -> u32 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 1,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        _ => 0
    }
}

fn value_of_hand2 (hand: &Hand) -> (u32, [u32; 5]) {
    let mut hand_type = 0;
    let mut card_values: [u32; 5] = [0; 5];
    let mut first: u32 = 0;
    let mut second: u32 = 0;
    let mut jokers: u32 = 0;
    for (index ,char) in hand.cards.chars().enumerate() {
        let value = value_of_card2(&char);
        card_values[index] = value;
        let count: u32 = card_values.iter().filter(|&&v|  v == value).count().try_into().unwrap();
        if value == 1 {
            jokers = jokers + 1
        }
        else if count > first {
            
            first = count;

        }
        else if count > second {
            second = count;
        }
        
        
    }
    first = first + jokers;
    
    
    match (first, second) {
        (1, 1) => hand_type = 0,
        (2, 1) => hand_type = 1,
        (2, 2) => hand_type = 2,
        (3, 1) => hand_type = 3,
        (3, 2) => hand_type = 4,
        (4, 1) => hand_type = 5,
        (5, 0) => hand_type = 6,
        (_, _) => ()
    }
    
    
    return (hand_type, card_values)
}

fn compare_hands2 (a: &Hand, b: &Hand) -> std::cmp::Ordering {
    let (t_a, c_a) = value_of_hand2(a);
    let (t_b, c_b) = value_of_hand2(b);
    if t_a < t_b {
        return std::cmp::Ordering::Less;
    }
    else if t_a > t_b {
        return std::cmp::Ordering::Greater;
    }
    else {
        return compare_value_cards(c_a, c_b)
    }
}

pub async fn solve_part_two () {
    println!("part 2");

    let data = get_day_7_data();
    let request = fetch(data.url, data.session_cookie).await;

    match request {
        Err(_) => {
            println!("Bad Request");
        }
        Ok(body) => {

            let data = body.text().await.unwrap();
            let mut hands: Vec<Hand> = vec![];

            data.lines().for_each(|line| {
                let line_data: Vec<&str> = line.split_whitespace().collect();
                let hand = Hand {
                    cards: line_data[0],
                    bid: line_data[1].parse().unwrap()
                };
                hands.push(hand);
            });

            hands.sort_by(|a, b| compare_hands2(a, b));

            

            let mut sum: u32 = 0;

            for (index, bid) in hands.iter().map(|hand| hand.bid ).enumerate() {
                sum = sum + bid * (index as u32 + 1);
                println!("cards: {}, bid: {}", hands[index as usize].cards, bid)
            }
            println!("sum: {}", sum)

        }
    }
}