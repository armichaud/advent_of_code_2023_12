use std::{io::{BufReader, BufRead}, fs::File};

const OPERATIONAL: char = '.';
const DAMAGED: char = '#';
const UNKOWN: char = '?';

#[derive(Debug)]
struct Record {
    springs: Vec<String>,
    damaged_spring_groupings: Vec<usize>,
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
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
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

fn get_arrangements(record: &Record) -> usize { 
    if record.damaged_spring_groupings.len() == 0 {
        return 1;
    }
    let next_damaged_spring_grouping = record.damaged_spring_groupings[0];
    let next_spring_group = &record.springs[0];
    if next_spring_group.len() < next_damaged_spring_grouping || next_spring_group.chars().nth(0).unwrap() == OPERATIONAL {
        return get_arrangements(&Record {
            springs: record.springs[1..].to_vec(),
            damaged_spring_groupings: record.damaged_spring_groupings.clone(),
        });
    }
    if next_spring_group.len() == next_damaged_spring_grouping && next_spring_group.chars().nth(0).unwrap() == DAMAGED {
        return get_arrangements(&Record {
            springs: record.springs[1..].to_vec(),
            damaged_spring_groupings: record.damaged_spring_groupings[1..].to_vec(),
        });
    }
    let mut arrangements = 0;

    arrangements
 }

fn solution(filename: &str) -> usize {
    let records = parse_input(filename);
    records.iter().map(|r| get_arrangements(r)).sum()
}

fn main() {
    assert_eq!(solution("example.txt"), 21);
    assert_eq!(solution("input.txt"), 0);
}
