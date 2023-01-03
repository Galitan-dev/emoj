use std::{env, io};

fn main() {
    let input = env::args().collect::<Vec<String>>()[1..].join(" ");
    
    if input.len() > 0 {
        return emojize(input);
    }

    let stdin = io::stdin();
    for line in stdin.lines() {
        emojize(line.unwrap());
    }
}

fn emojize(input: String) {
    println!("{input}");
}