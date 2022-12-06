use std::convert::TryInto;

fn rucksack2compartments(rucksack: &str, n_compartments: Option<u32>) -> Vec<String> {
    let n: usize= n_compartments.unwrap_or(2).try_into().unwrap(); // set default compartments = 2
    //let len = rucksack.chars().count();
    //let len: u32 = rucksack.chars().count().try_into().unwrap();
    let len: usize = rucksack.len().try_into().unwrap();
    let compartment_len = len / n;
    let mut string = String::new();
    let mut outv = Vec::<String>::new();
    for (i,c) in rucksack.chars().enumerate() {
        //println!("{} {}", i, c);

        string.push(c);
        if (i+1) % compartment_len == 0 {
            outv.push(string.clone());
            string="".to_string();
        }
    }
    //("foo".to_string(),"bar".to_string())
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
