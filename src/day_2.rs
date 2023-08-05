use std::fs::read_to_string;
use crate::day_2::HandShape::*;
use crate::day_2::Outcome::*;

#[derive(PartialEq, Clone, Copy)]
pub enum HandShape {
    Rock = 1,
    Paper,
    Scissors
}

#[derive(PartialEq, Clone, Copy)]
pub enum Outcome {
    Lose = 1,
    Draw,
    Win
}

pub fn read_lines_day_2_part_1(filename: &str) -> Vec<Vec<HandShape>> {
    let mut result: Vec<Vec<HandShape>> = Vec::new();
    let mut current_grouping: Vec<HandShape> = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        for shape in line.split(' ') {
            match shape{
                "A"|"X"=>current_grouping.push(Rock),
                "B"|"Y"=>current_grouping.push(Paper),
                "C"|"Z"=>current_grouping.push(Scissors),
                _=>print!("Uh oh!")
            }
        }
        result.push(current_grouping);
        current_grouping = Vec::new();
    }

    result
}

pub fn calculate_day_2_part_1(rounds: Vec<Vec<HandShape>>) -> i32 {
    let mut result = 0;
    for hands in rounds {
        result += get_score(hands[1]);
        result += get_win(hands[1], hands[0]);
    }
    result
}

pub fn read_lines_day_2_part_2(filename: &str) -> Vec<(HandShape, Outcome)> {
    let mut result: Vec<(HandShape, Outcome)> = Vec::new();
    let mut current_handshape = Rock;
    let mut current_outcome = Lose;

    for line in read_to_string(filename).unwrap().lines() {
        let mut round = line.split(' ');
        match round.next(){
            Some("A")=>current_handshape = Rock,
            Some("B")=>current_handshape = Paper,            
            Some("C")=>current_handshape = Scissors,
            _=>print!("Uh oh!")
        }
        match round.next(){
            Some("X")=>current_outcome = Lose,
            Some("Y")=>current_outcome = Draw,            
            Some("Z")=>current_outcome = Win,
            _=>print!("Uh oh!")
        }
        result.push((current_handshape, current_outcome));
    }

    result
}

pub fn calculate_day_2_part_2(rounds: Vec<(HandShape, Outcome)>) -> i32 {
    let mut result = 0;
    for round in rounds {
        let hand = get_hand(round.0, round.1);
        result += get_score(hand);
        result += get_win(hand, round.0);
    }
    result
}

fn get_score(shape: HandShape) -> i32 {
    match shape{
        HandShape::Rock=>1,
        HandShape::Paper=>2,
        HandShape::Scissors=>3
    }
}

fn get_win(shape1: HandShape, shape2: HandShape) -> i32 {
    if beats(shape1, shape2) {
        6
    } else {
        if beats(shape2, shape1) {
            0
        } else {
            3
        }
    }
}

fn beats(shape1: HandShape, shape2: HandShape) -> bool {
    match shape1{
        Rock=>shape2==Scissors,
        Paper=>shape2==Rock,
        Scissors=>shape2==Paper
    }
}

fn get_hand(shape1: HandShape, outcome: Outcome) -> HandShape {
    match outcome{
        Lose=>match shape1{
            Rock=>Scissors,
            Paper=>Rock,
            Scissors=>Paper 
        }
        Draw=>shape1,
        Win=>match shape1{
            Rock=>Paper,
            Paper=>Scissors,
            Scissors=>Rock 
        }
    }
}