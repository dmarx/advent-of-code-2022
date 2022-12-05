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


enum RpsWeapon {
    Rock,
    Paper,
    Scissors
}

enum RpsOutcome {
    Win,
    Lose,
    Tie
}



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

fn evaluate_rps_score(left: &RpsWeapon, right: &RpsWeapon) -> u32 {
    let outcome =  evaluate_rps_outcome(&left, &right);
    let mut score: u32 = weapon2score(&left);
    score += match outcome {
        RpsOutcome::Win => 6,
        RpsOutcome::Tie => 3,
        RpsOutcome::Lose => 0
    };
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
        let left = decode_left(token_left);
        let right = decode_right(token_right);
        //let outcome = evaluate_rps_outcome(&left, &right);
        let outcome = evaluate_rps_outcome(&right, &left);

        match outcome {
            RpsOutcome::Win => println!("Win"),
            RpsOutcome::Lose => println!("Lose"),
            RpsOutcome::Tie => println!("Tie")
        }
        let score = evaluate_rps_score(&right, &left);
        //println!("{}", score)
        running_score += score;
        println!("{} {}", running_score, score);
    }

}




fn main() {
    println!("Hello, world!");
    part1_soln();
}
