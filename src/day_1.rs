use std::fs::read_to_string;

pub fn read_lines_day_1(filename: &str) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    let mut current_grouping: Vec<i32> = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        if line.to_string() == "" {
            result.push(current_grouping);
            current_grouping = Vec::new();
        } else {
            current_grouping.push(line.parse::<i32>().unwrap());
        }
    }
    result.push(current_grouping);

    result
}

pub fn calculate_day_1(elves: Vec<Vec<i32>>, num_elves: i32) -> i32 {
    let mut top_elves: Vec<i32> = Vec::new();
    
    for elf in elves {
        let cals = sum_calories(elf);
        match top_elves.len() < num_elves as usize {
            true=>{
                top_elves.push(cals);
                top_elves.sort();
                top_elves.reverse();
            },
            false=>{
                if cals > *top_elves.last().unwrap() {
                    top_elves.push(cals);
                    top_elves.sort();
                    top_elves.reverse();
                    top_elves.truncate(num_elves as usize);
                }
            }
        }
    }

    return sum_calories(top_elves);
}

fn sum_calories(food: Vec<i32>) -> i32 {
    let mut sum = 0;
    
    for item in food {
        sum += item;
    }
    
    sum
}
