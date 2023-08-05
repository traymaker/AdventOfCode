use std::fs::read_to_string;
use std::collections::HashSet;

pub fn read_lines_day_3(filename: &str) -> Vec<Vec<char>> {
    let mut result: Vec<Vec<char>> = Vec::new();
    let mut current_rucksack: Vec<char> = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        for letter in line.chars() {
            current_rucksack.push(letter);
        }
        result.push(current_rucksack);
        current_rucksack = Vec::new();
    }

    result
}

pub fn calculate_day_3(rucksacks: Vec<Vec<char>>) -> u32 {
    let mut sum: u32 = 0;
    for rucksack in rucksacks {
        sum += get_value(find_duplicate(rucksack))
    }
    return sum;
}

fn find_duplicate(rucksack: Vec<char>) -> char {
    let mut half: HashSet<&char> = HashSet::new();
    for i in rucksack.len()/2..rucksack.len() {
        half.insert(rucksack.get(i).expect("AHHH"));
    }
    for i in 0..rucksack.len()/2 {
        let letter = rucksack.get(i).expect("AHHH");
        if half.contains(&letter) {
            return *letter
        }
    }
    return 'a';
}

fn get_value(letter: char) -> u32 {
    let value = letter as u32;
    //conversion from ASCII value to priority: a-z => 1-26, A-Z => 27-52
    match value > 90{
        true=>value-96,
        false=>value-38,
    }
}