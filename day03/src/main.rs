use std::collections::hash_map::RandomState;
//use std::collections::hash_map::Intersection;
use std::convert::TryInto;
use std::collections::HashSet;

// let a = HashSet::from([1, 2, 3]);
// let b = HashSet::from([4, 2, 3, 4]);

// let mut intersection = a.intersection(&b);

fn rucksack2compartments(rucksack: &str, n_compartments: Option<u32>) -> Vec<String> {
    let n: usize= n_compartments.unwrap_or(2).try_into().unwrap(); // set default compartments = 2
    let compartment_len = rucksack.len() / n;
    let mut compartment = String::new();
    let mut outv = Vec::<String>::new(); // vector of compartments
    for (i,c) in rucksack.chars().enumerate() {
        compartment.push(c);
        if (i+1) % compartment_len == 0 {
            outv.push(compartment.clone());
            compartment="".to_string();
        }
    }
    outv
}

fn char2priority(c: &char) -> u32 {
    for (i, v) in "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string().chars().enumerate() {
        if c == &v {
            let j: u32 = i.try_into().unwrap();
            return j+1;
        }
    }
    unreachable!()
}

// fn common_items(compartments: Vec<String>) -> String {
//     let mut common = String::new();
//     for (i, c1) in compartments.iter().enumerate() {
//         if i == 0 {
//             let c0 = compartments[i];
//             //let c0_chars = c0.chars();
//             let hs0 = HashSet::new();
//             for c_j in c0.chars() {
//                 hs0.push(c_j);
//             }
//             continue
//         }
//         //let c0 = compartments[i-1];
//         let a = HashSet::from([c0.chars()]);
//         let b = HashSet::from([c1.chars()]);
//         let mut ab_common = a.intersection(&b);
//         if i == 1 {
//             common = ab_common.clone();
//         } else if i > 1 {
//             //let c = HashSet::from(ab_common);
//             //let d = 
//             common = ab_common.intersection(common);
//         }
//     }
//     common
// }

fn common_items_binary(compartments: Vec<String>) -> String {
    let c0 = compartments[0].clone();
    let c1 = compartments[1].clone();
    let hs0: HashSet<char> = HashSet::from_iter(c0.chars());
    let hs1: HashSet<char> = HashSet::from_iter(c1.chars());
    let common: String = hs0.intersection(&hs1).collect();
    //let outv: String = common.collect();
    common
}

fn tests() {
    println!("Hello, world!");
    let outv = rucksack2compartments("foodbarf", Some(2));
    println!("{}", outv.len());
    // for compartment in outv {
    //     println!("{}", compartment);
    // }
    let common = common_items_binary(outv);
    println!("{}", char2priority(&'A'));
    println!("{}", common);
}

fn main() {
    tests()
}