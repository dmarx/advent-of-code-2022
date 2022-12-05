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



fn evaluate_rps(left: RpsWeapon, right: RpsWeapon) -> RpsOutcome {
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



fn part1_soln () {
    let fp = File::open(Path::new("../data/input-day01.txt")).unwrap();
    let file = BufReader::new(&fp);
    ////////////
    let left = decode_left("A");
    let right = decode_right("X");
    let outv = evaluate_rps(left, right);
    
    match outv {
        RpsOutcome::Win => println!("Win"),
        RpsOutcome::Lose => println!("Lose"),
        RpsOutcome::Tie => println!("Tie")
    }
}




fn main() {
    println!("Hello, world!");
    part1_soln();
}
