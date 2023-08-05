mod day_1;
mod day_2;

fn main() {
    println!("{}", day_1::calculate_day_1(day_1::read_lines_day_1("./data/day_1.txt"), 3));
    println!("{}", day_2::calculate_day_2_part_1(day_2::read_lines_day_2_part_1("./data/day_2.txt")));
    println!("{}", day_2::calculate_day_2_part_2(day_2::read_lines_day_2_part_2("./data/day_2.txt")));
}