use std::io::{BufReader, BufRead};
use std::fs::File;
use std::path::Path;

//use std::fs;
//use std::net::SocketAddr;

// https://doc.rust-lang.org/std/fs/fn.read.html

// fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
//     let foo: SocketAddr = String::from_utf8_lossy(&fs::read("../data/input-day01.txt")?).parse()?;
//     Ok(())
    
// }


// https://github.com/rochacbruno/py2rs#files

fn part1_soln () -> u32 {
    let fp = File::open(Path::new("../data/input-day01.txt")).unwrap();
    let file = BufReader::new(&fp);
    let mut accumulator: u32 = 0;
    let mut maxval: u32 = 0;
    for line in file.lines() {
        //  Iterate over lines
        let value_str: String = line.unwrap();
        if value_str.chars().count() > 0 {
            // have a value, accumulate
            let value_int: u32 = value_str.parse().unwrap();
            accumulator += value_int;
        } else {
            // empty line, test max and reset
            if accumulator > maxval {
                maxval = accumulator;
                //println!("{}", maxval);
            }
            accumulator = 0;
        }
    }
    maxval
}

fn main () {
    let soln1: u32 = part1_soln();
    println!("Part-01 solution: {}", soln1);
}