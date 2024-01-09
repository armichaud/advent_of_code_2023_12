use std::{io::{BufReader, BufRead}, fs::File, collections::HashMap};

const OPERATIONAL: char = '.';
const DAMAGED: char = '#';
const UNKNOWN: char = '?';

#[derive(Debug, Clone)]
struct Record {
    full_record: String,
    springs: Vec<String>,
    damaged_spring_groupings: Vec<usize>,
}

fn parse_input(filename: &str) -> Vec<Record> {
    let file = File::open(filename).unwrap();
    BufReader::new(file).lines().map(|l| {
        let line = l.unwrap();
        let mut split = line.split_whitespace();
        let full_record = split.next().unwrap().to_string();
        let damaged_spring_groupings = split
            .next()
            .unwrap()
            .trim()
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let mut springs = Vec::new();
        let mut current_group = String::new();
        for (i, c) in full_record.chars().enumerate() {
            if i > 0 && c != full_record.chars().nth(i - 1).unwrap() {
                springs.push(current_group.clone());
                current_group.clear();
            }
    
            current_group.push(c);
        }
    
        if !current_group.is_empty() {
            springs.push(current_group);
        }
        Record {
            full_record,
            springs,
            damaged_spring_groupings,
        }
    }).collect::<Vec<Record>>()
}

fn is_valid_arrangement(arrangement: String, record: Record) -> bool {
    let mut groups = record.damaged_spring_groupings;
    let mut flag = false;
    for c in arrangement.chars() {
        if !flag && c == DAMAGED {
            flag = true;
            if groups.is_empty() || groups[0] == 0 {
                return false;
            }
            groups[0] -= 1;
            continue;
        }
        if flag && c == OPERATIONAL {
            if groups[0] != 0 {
                return false;
            }
            groups.remove(0);
            flag = false;
            continue;
        }
        if flag && c == DAMAGED {
            if groups[0] == 0 {
                return false;
            }
            groups[0] -= 1;
            continue;
        }
    }
    groups.is_empty() || groups == vec![0]
}

fn get_all_permutations(n: usize, memo: &mut HashMap<usize, Vec<String>>) -> Vec<String> {
    if memo.contains_key(&n) {
        return memo.get(&n).unwrap().clone();
    }
    let mut permutations = Vec::new();
    for p in get_all_permutations(n - 1, memo) {
        permutations.push(format!("{}{}", OPERATIONAL, p));
        permutations.push(format!("{}{}", DAMAGED, p));
    }
    permutations
}

fn get_arrangements(working_arrangement: String, record: &Record, memo: &mut HashMap<usize, Vec<String>>) -> usize {
    if working_arrangement.len() == record.full_record.len() {
        return if is_valid_arrangement(working_arrangement, record.clone()) { 1 } else { 0 };
    }
    let mut arrangements = 0;
    let mut remaining_springs = record.springs.clone();
    let spring = remaining_springs.remove(0);
    let new_record = Record {
        full_record: record.full_record.clone(),
        springs: remaining_springs,
        damaged_spring_groupings: record.damaged_spring_groupings.clone(),
    };
    if spring.chars().next().unwrap() == UNKNOWN {
        for p in get_all_permutations(spring.len(), memo) {

            arrangements += get_arrangements(format!("{}{}", working_arrangement, p), &new_record, memo);
        }
    } else {
        arrangements += get_arrangements(format!("{}{}", working_arrangement, spring), &new_record, memo);
    }
    arrangements
}

fn solution(filename: &str) -> usize {
    let records = parse_input(filename);
    let mut m = HashMap::new();
    m.insert(1, vec![OPERATIONAL.to_string(), DAMAGED.to_string()]);
    records.iter().map(|r| get_arrangements(String::new(), r, &mut m)).sum()
}

fn main() {
    println!("{}", solution("example.txt"));
    println!("{}", solution("input.txt"));
}
