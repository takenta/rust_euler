use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new("./resources/p022_names.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut buf = String::new();
    let mut names: Vec<&str> = match file.read_to_string(&mut buf) {
        Err(why) => panic!("couldn't read {}: {}", display, why.description()),
        Ok(_) => buf.split(",")
                    .map(|x| x.trim_matches('\"'))
                    .collect(),
    };
    names.sort();

    let mut acc: u32 = 0;
    for i in 0..names.len() {
        acc = acc + (i + 1) as u32 * calc_score(names[i]);
    };

    println!("{}", acc);
}

fn calc_score(name: &str) -> u32 {
    name.clone()
        .chars()
        .map(|c| c as u8 - 'A' as u8 + 1)
        .sum::<u8>() as u32
}
