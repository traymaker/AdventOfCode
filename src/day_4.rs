use std::fs::read_to_string;

pub fn read_lines_day_4(filename: &str) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    let mut current_assignment: Vec<i32> = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        for assignment in line.split(',') {
            for digit in assignment.split('-') {
                current_assignment.push(digit.parse::<i32>().unwrap());
            }
        }
        result.push(current_assignment);
        current_assignment = Vec::new();
    }

    result
}

pub fn calculate_day_4_part_1(all_assignments: Vec<Vec<i32>>) -> i32 {
    let mut sum: i32 = 0;

    for assignment in all_assignments {
        if contains(assignment) {
            sum += 1;
        }
    }

    sum
}

pub fn calculate_day_4_part_2(all_assignments: Vec<Vec<i32>>) -> i32 {
    let mut sum: i32 = 0;

    for assignment in all_assignments {
        if overlaps(assignment) {
            sum += 1;
        }
    }

    sum
}

fn contains(assignment: Vec<i32>) -> bool {
    if assignment.get(0) == assignment.get(2) {
        return true;
    }
    match assignment.get(0) >= assignment.get(2){
        true=>assignment.get(1) <= assignment.get(3),
        false=>assignment.get(1) >= assignment.get(3),
    }
}

fn overlaps(assignment: Vec<i32>) -> bool {
    (assignment.get(0) <= assignment.get(3) &&
        assignment.get(1) >= assignment.get(2))
    ||
    (assignment.get(2) <= assignment.get(1) &&
        assignment.get(3) >= assignment.get(0))
}
