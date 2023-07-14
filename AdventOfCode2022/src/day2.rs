use std::collections::HashMap;
use std::fs;
use std::vec::Vec;

pub fn main() {
    // --snip--
    println!("In file {}", "./day2_input.txt");

    let contents =
        fs::read_to_string("./day2_input.txt").expect("Should have been able to read the file");
    let mut plays = Vec::new();
    let mut points;
    let mut score = 0;
    let selected_score = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);

    println!("Am entering loop now:");
    for text in contents.split('\n') {
        plays = text.split(" ").collect();
        println!("{:?}", plays);    
        points = decide_game(plays[0].to_owned(), plays[1].to_owned())
            + selected_score.get(plays[1]).unwrap_or(&0);
        println!("Points this round {points}");
        score += points;
    }

    println!("The total score is: {score}");
}

fn decide_game(opp: String, p: String) -> i32 {
    let mut result = -1;
    if opp == "A" {
        if p == "X" {
            result = 3
        };
        if p == "Y" {
            result = 6
        };
        if p == "Z" {
            result = 0
        };
    } else if opp == "B" {
        if p == "X" {
            result = 0
        };
        if p == "Y" {
            result = 3
        };
        if p == "Z" {
            result = 6
        };
    } else if opp == "C" {
        if p == "X" {
            result = 6
        };
        if p == "Y" {
            result = 0
        };
        if p == "Z" {
            result = 3
        };
    }
    return result;
}
