use std::{fs::File, io::{BufRead, BufReader}};

fn main() {
    println!("Solution to question number 1: {}", ex1());
}

fn ex1() -> i32{
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut total = 0;

    for line in reader.lines() {
        let numbers: Vec<i32> = line.unwrap().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
        if numbers.len() < 2 {
            total += 1;
            continue;
        }
        if numbers[0] == numbers[1] {continue;}

        let mut direction = 1;
        if numbers[0] > numbers[1] {direction = -1}
        let mut prev = numbers[0];
        let mut safe = true;

        for n in numbers[1..].iter() {
            if ! (prev * direction < n * direction ) || (prev - n).abs() < 1 || (prev - n).abs() > 3 {
                safe = false;
                break;
            }

            prev = *n;
        }

        if safe {total += 1;}
        
    }
    total
}
