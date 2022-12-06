use std::{
    env,
    fs::{
        read_to_string,
    }
};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        panic!("must provide an input file")
    }
    
    let content = read_to_string(&args[1]).unwrap();
    
    let mut elves: Vec<usize> = Vec::new();
    elves.push(0);
    
    for line in content.lines() {
        let line = line.trim();
        if line.len() == 0 {
            elves.push(0);
        }
        else if let Some(last) = elves.last_mut() {
            *last += line.parse::<usize>().unwrap();
        }
    }
    elves.sort();
    elves.reverse();
    
    // Part 1
    println!("{:?}", elves[0]);
    
    // Part 2
    println!("{:?}", elves[0] + elves[1] + elves[2]);
}
