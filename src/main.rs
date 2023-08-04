fn main() {
    println!("{}", day_1());
}

fn day_1() -> i64 {
    let elves: Vec<Vec<i64>> = vec![vec![1000, 2000, 3000], vec![4000], vec![5000, 6000], vec![7000, 8000, 9000], vec![10000]];
    let mut max_cal = 0;
    for elf in elves {
        let cals = sum_calories(elf);
        if cals > max_cal {
            max_cal = cals;
        }
    }
    return max_cal;
}

fn sum_calories(food: Vec<i64>) -> i64 {
    let mut sum = 0;
    for item in food {
        sum += item;
    }
    return sum;
}