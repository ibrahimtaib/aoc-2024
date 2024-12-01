use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::iter::zip;
use std::vec;

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut list1 = vec![];
    let mut list2 = vec![];

    for line in reader.lines() {
        let numbers: Vec<i32> = line.unwrap().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();

        list1.push(numbers[0]);
        list2.push(numbers[1]);
    }

    list1.sort();
    list2.sort();

    println!("{}", zip(list1, list2).fold(0, |acc, (a, b)| acc +  (a - b).abs()));
    Ok(())
} 