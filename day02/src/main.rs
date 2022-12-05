use std::io::{BufReader, BufRead};
use std::fs::File;
use std::path::Path;
//use std::collections::HashMap;


fn decode_left (val: &str) -> RpsWeapon {
    let outv: RpsWeapon = match val {
        "A" => RpsWeapon::Rock,
        "B" => RpsWeapon::Paper,
        "C" => RpsWeapon::Scissors,
        &_ => todo!()
    };
    outv
}

fn decode_right (val: &str) -> RpsWeapon {
    let outv: RpsWeapon = match val {
        "X" => RpsWeapon::Rock,
        "Y" => RpsWeapon::Paper,
        "Z" => RpsWeapon::Scissors,
        &_ => todo!()
    };
    outv
}

#[derive(Clone)]
enum RpsWeapon {
    Rock,
    Paper,
    Scissors
}

#[derive(PartialEq, Eq, Debug)]
enum RpsOutcome {
    Win,
    Lose,
    Tie
}


//left = me, right = opponent
fn evaluate_rps_outcome(left: &RpsWeapon, right: &RpsWeapon) -> RpsOutcome {
    let outv: RpsOutcome = match left {
        RpsWeapon::Rock =>  match right {
            RpsWeapon::Rock => RpsOutcome::Tie,
            RpsWeapon::Paper => RpsOutcome::Lose,
            RpsWeapon::Scissors => RpsOutcome::Win
        }   
        RpsWeapon::Paper => match right {
            RpsWeapon::Rock => RpsOutcome::Win,
            RpsWeapon::Paper => RpsOutcome::Tie,
            RpsWeapon::Scissors => RpsOutcome::Lose
        }
        RpsWeapon::Scissors => match right {
            RpsWeapon::Rock => RpsOutcome::Lose,
            RpsWeapon::Paper => RpsOutcome::Win,
            RpsWeapon::Scissors => RpsOutcome::Tie
        }
    };
    outv
}

fn weapon2score(weapon: &RpsWeapon) -> u32 {
    match weapon {
        RpsWeapon::Rock => 1,
        RpsWeapon::Paper => 2,
        RpsWeapon::Scissors => 3
    }
}

fn outcome2score(outcome: &RpsOutcome) -> u32 {
    match outcome {
        RpsOutcome::Win => 6,
        RpsOutcome::Tie => 3,
        RpsOutcome::Lose => 0
    }
}

fn evaluate_rps_score(left: &RpsWeapon, right: &RpsWeapon) -> u32 {
    let outcome =  evaluate_rps_outcome(&left, &right);
    let mut score: u32 = weapon2score(&left);
    score += outcome2score(&outcome);
    score
}


fn part1_soln () {
    //let fp = File::open(Path::new("../data/input-day02-test.txt")).unwrap();
    let fp = File::open(Path::new("../data/input-day02.txt")).unwrap();
    let file = BufReader::new(&fp);
    ////////////
    let mut running_score: u32 = 0;
    for line in file.lines() {
        let value_str: String = line.unwrap();
        let mut tokens = value_str.split_whitespace();
        let token_left = tokens.next().unwrap();
        let token_right = tokens.next().unwrap();
        // left = opponent (unnecessarily confusing!)
        let left = decode_left(token_left);
        //right = me
        let right = decode_right(token_right);
        //let outcome = evaluate_rps_outcome(&left, &right);
        //let outcome = evaluate_rps_outcome(&right, &left);

        // match outcome {
        //     RpsOutcome::Win => println!("Win"),
        //     RpsOutcome::Lose => println!("Lose"),
        //     RpsOutcome::Tie => println!("Tie")
        // }
        let score = evaluate_rps_score(&right, &left);
        //println!("{}", score)
        running_score += score;
        //println!("{} {}", running_score, score);
    }
    println!("{}", running_score);
}

fn decode_right_outcome (val: &str) -> RpsOutcome {
    let outv: RpsOutcome = match val {
        "X" => RpsOutcome::Lose,
        "Y" => RpsOutcome::Tie,
        "Z" => RpsOutcome::Win,
        &_ => todo!()
    };
    outv
}

fn infer_my_weapon (opponent: &RpsWeapon, nec_outcome: &RpsOutcome) -> RpsWeapon {
    //let mut outv: RpsWeapon = RpsWeapon::Rock; // this should init empty
    for weapon in [RpsWeapon::Rock, RpsWeapon::Paper, RpsWeapon::Scissors] {
        //if &evaluate_rps_outcome(&opponent, &weapon) == nec_outcome {
        if &evaluate_rps_outcome(&weapon, &opponent) == nec_outcome {
            return weapon;
        }
    }
    // thanks reddit! 
    // https://www.reddit.com/r/rust/comments/zdao9b/noob_help_cant_get_around_type_errors_in_simple/
    unreachable!()
}


fn part2_soln () {
    //let fp = File::open(Path::new("../data/input-day02-test.txt")).unwrap();
    let fp = File::open(Path::new("../data/input-day02.txt")).unwrap();
    let file = BufReader::new(&fp);
    ////////////
    let mut running_score: u32 = 0;
    for line in file.lines() {
        let value_str: String = line.unwrap();
        let mut tokens = value_str.split_whitespace();
        let token_left = tokens.next().unwrap();
        let token_right = tokens.next().unwrap();
        //println!("{} {}", &token_left, &token_right);

        let opponent_weapon = decode_left(token_left);
        let outcome = decode_right_outcome(&token_right);
        let my_weapon = infer_my_weapon(&opponent_weapon, &outcome);

        //let score = evaluate_rps_score(&outcome, &left);
        let weapon_score = weapon2score(&my_weapon);
        let outcome_score = outcome2score(&outcome);
        //println!("{} {}", weapon_score, outcome_score);
        //running_score += weapon2score(&my_weapon);
        //running_score +=  outcome2score(&outcome);
        running_score += weapon_score + outcome_score;
        //println!("{} {}", running_score, score);
    };
    println!("part2: {}", running_score);
}

fn main() {
    println!("Hello, world!");
    part1_soln();
    part2_soln();
}
