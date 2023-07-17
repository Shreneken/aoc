use std::collections::HashMap;
use std::fs;
use std::vec::Vec;

// --snip--
pub fn main() {
    println!("In file {}", "./day2_input.txt");
    let contents =
        fs::read_to_string("./day2_input.txt").expect("Should have been able to read the file");
    let mut plays = Vec::new();
    let mut points;
    let mut score = 0;
    let selected_score_pt2 = HashMap::from([("X", 0), ("Y", 3), ("Z", 6)]);

    println!("Am entering loop now:");
    for text in contents.split('\n') {
        plays = text.split(" ").collect();
        println!("{:?}", plays);
        points = decide_game_pt2(plays[0].to_owned(), plays[1].to_owned())
            + selected_score_pt2.get(plays[1]).unwrap_or(&0);
        println!("Points this round {points}");
        score += points;
    }

    println!("The total score is: {score}");
}

fn decide_game_pt1(opp: String, p: String) -> i32 {
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

fn decide_game_pt2(opp: String, p_res: String) -> i32 {
    // p_res gives result points, we need to find points for the strategy picked
    // 1 for rock, 2 for paper and 3 for scissors
    let mut result = -1;
    if opp == "A" {
        //opp picked rock
        if p_res == "X" {
            // we need to lose, so we pick scissors
            result = 3;
        };
        if p_res == "Y" {
            //we need to draw, so we pick rock
            result = 1;
        };
        if p_res == "Z" {
            // we need to win, so we pick paper
            result = 2;
        };
    } else if opp == "B" {
        //opp picked paper
        if p_res == "X" {
            // we need to lose, so we pick rock
            result = 1;
        };
        if p_res == "Y" {
            // we need to draw, so we pick paper
            result = 2;
        };
        if p_res == "Z" {
            // we need to win, so we pick scissors
            result = 3;
        };
    } else if opp == "C" {
        if p_res == "X" {
            // opp picked scissors
            result = 2; //  we need to lose, so we pick paper
        };
        if p_res == "Y" {
            // we need to draw, so we pick scissors
            result = 3;
        };
        if p_res == "Z" {
            // we need to win, so we pick rock
            result = 1;
        };
    }

    return result;
}
