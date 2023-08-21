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

pub fn calculate_day_3_part_1(rucksacks: Vec<Vec<char>>) -> u32 {
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

pub fn calculate_day_3_part_2(rucksacks: Vec<Vec<char>>) -> u32 {
    let mut sum: u32 = 0;

    for i in (0..rucksacks.len()).step_by(3) {
        let badge: char = three_way_intersection(
            build_set(rucksacks.get(i).expect("Uh oh")), 
            build_set(rucksacks.get(i+1).expect("Uh oh")),
            build_set(rucksacks.get(i+2).expect("Uh oh"))
        );

        sum += get_value(badge);
    }

    return sum;
}

fn build_set(rucksack: &Vec<char>) -> HashSet<char> {
    let mut rucksack_set = HashSet::new();

    for letter in rucksack {
        rucksack_set.insert(*letter);
    }

    return rucksack_set;
}

fn three_way_intersection(rucksack1: HashSet<char>, rucksack2: HashSet<char>, rucksack3: HashSet<char>) -> char {
    for letter in rucksack1 {
        if rucksack2.contains(&letter) && rucksack3.contains(&letter) {
            return letter
        }
    }
    panic!("No badge found");
}

fn get_value(letter: char) -> u32 {
    let value = letter as u32;
    //conversion from ASCII value to priority: a-z => 1-26, A-Z => 27-52
    match value > 90{
        true=>value-96,
        false=>value-38,
    }
}