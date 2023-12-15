use std::{io::{BufReader, BufRead}, fs::File};

const OPERATIONAL: char = '.';
const DAMAGED: char = '#';
const UNKOWN: char = '?';

#[derive(Debug)]
struct Record {
    springs: Vec<String>,
    damaged_spring_groupings: Vec<u32>,
}

fn parse_input(filename: &str) -> Vec<Record> {
    let file = File::open(filename).unwrap();
    BufReader::new(file).lines().map(|l| {
        let line = l.unwrap();
        let mut split = line.split_whitespace();
        let spring_str = split.next().unwrap().to_string();
        let damaged_spring_groupings = split
            .next()
            .unwrap()
            .trim()
            .split(',')
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        let mut springs = Vec::new();
        let mut current_group = String::new();
        for (i, c) in spring_str.chars().enumerate() {
            if i > 0 && c != spring_str.chars().nth(i - 1).unwrap() {
                springs.push(current_group.clone());
                current_group.clear();
            }
    
            current_group.push(c);
        }
    
        if !current_group.is_empty() {
            springs.push(current_group);
        }
        Record {
            springs,
            damaged_spring_groupings,
        }
    }).collect::<Vec<Record>>()
}

fn main() {
    println!("{:?}", parse_input("example.txt"));
}
