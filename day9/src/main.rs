use std::{fs::read_to_string, io::stdin};

enum ExtrapolateType {
    First,
    Last
}

fn main() {
    let inputs: String = read_to_string("input.txt").expect("File with inputs");
    let sum = part1(&inputs);
    let sum2 = part2(&inputs);
    println!("Sum 1: {sum}");
    println!("Sum 2: {sum2}");
}

fn part1(inputs: &str) -> i64 {
    let mut parsed_file = parse_file(inputs);
    let new_histories = parsed_file
        .iter_mut()
        .map(|history| { 
            let derived_history = calc_difference(history);
            history.push(derived_history.last().unwrap() + history.last().unwrap());
            history.to_owned()
        }).collect::<Vec<Vec<i64>>>();
    println!("{new_histories:?}");
    new_histories
        .iter()
        .fold(0, |acc, l| l.last().unwrap() + acc)
}

fn part2(inputs: &str) -> i64 {
    let mut parsed_file = parse_file(inputs);
    let new_histories = parsed_file
        .iter_mut()
        .map(|history| { 
            let derived_history = calc_difference_2(history);
            history.insert(0, history.first().unwrap()- derived_history.first().unwrap());
            history.to_owned()
        }).collect::<Vec<Vec<i64>>>();
    println!("{new_histories:?}");
    new_histories
        .iter()
        .fold(0, |acc, l| l.first().unwrap() + acc)
}

fn parse_file(file: &str) -> Vec<Vec<i64>> {
    file
        .lines()
        .map(|line| line
            .to_string()
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect::<Vec<i64>>()
            
        )
        .collect::<Vec<Vec<i64>>>()
}

fn calc_difference (history: &Vec<i64>) -> Vec<i64> {

    if history.windows(2).all(|window|window[0] == window[1]) {return vec![0; history.len()-1]}
    
    let mut calc_history = history.windows(2).map(|win| win[1] - win[0]).collect::<Vec<i64>>();

    let derived_history = calc_difference(&calc_history);

    calc_history.push(derived_history.last().unwrap() + calc_history.last().unwrap());

    calc_history

}

fn calc_difference_2 (history: &Vec<i64>) -> Vec<i64> {

    if history.windows(2).all(|window|window[0] == window[1]) {return vec![0; history.len()-1]}
    
    let mut calc_history = history.windows(2).map(|win| win[1] - win[0]).collect::<Vec<i64>>();

    let derived_history = calc_difference_2(&calc_history);

    calc_history.insert(0, calc_history.first().unwrap()- derived_history.first().unwrap());

    // println!("Calc : {calc_history:?}");
    calc_history

}