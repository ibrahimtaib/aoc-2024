use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;
use std::vec;

fn main() {
    println!("Solution to question number 1: {}", ex1());
    println!("Solution to question number 2: {}", ex2());
} 

fn ex1() -> i32 {
    let file = File::open("input.txt").unwrap();
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

    zip(list1, list2).fold(0, |acc, (a, b)| acc +  (a - b).abs())
}

fn ex2() -> i32{
    let mut counter: HashMap<i32, (i32, i32)> = HashMap::new();
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let numbers: Vec<i32> = line.unwrap().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();

        counter.entry(numbers[0]).or_insert((0, 0)).0 +=1;
        counter.entry(numbers[1]).or_insert((0, 0)).1 +=1;
    }

    counter.into_iter().fold(0, |acc, (key, (left, right))| key*left*right + acc)
}