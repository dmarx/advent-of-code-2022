use std::convert::TryInto;


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


fn main() {
    println!("Hello, world!");
    let outv = rucksack2compartments("foobar", Some(2));
    println!("{}", outv.len());
    for compartment in outv {
        println!("{}", compartment);
    }
}
