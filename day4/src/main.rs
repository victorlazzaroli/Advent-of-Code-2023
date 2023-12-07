use std::fs::read_to_string;
fn main() {
    let inputs: String = read_to_string("input.txt").expect("File with inputs");
    let sum = part1(&inputs);
    let sum2 = part2(&inputs);
    println!("Sum 1: {sum}");
    println!("Sum 2: {sum2}");
}

fn part1(inputs: &str) -> u32 {
    let lists:Vec<(String, String)> = inputs.lines().map(|l| remove_gamestring(l)).map(|l| divide_lists(&l)).collect();

    
    let winning_nums = lists.iter().map(|l| divide_numbers(&l.0.trim())).collect::<Vec<Vec<u16>>>();
    let scratch_nums = lists.iter().map(|l| divide_numbers(&l.1.trim())).collect::<Vec<Vec<u16>>>();
    // println!("{winning_nums:?}");
    let win_nums:Vec<Vec<u16>> = winning_nums.iter().zip(scratch_nums).map(|game| get_win_nums(&game.0, &game.1)).collect();
    
    win_nums.iter().fold(0, |acc, curr| calculate_points(&curr) + acc)
    
}

fn part2(inputs: &str) -> u32 {
    let lists:Vec<(String, String)> = inputs.lines().map(|l| remove_gamestring(l)).map(|l| divide_lists(&l)).collect();
    let mut scratch_cards_pile = vec![1; lists.len()];

    let winning_nums = lists.iter().map(|l| divide_numbers(&l.0.trim())).collect::<Vec<Vec<u16>>>();
    let scratch_nums = lists.iter().map(|l| divide_numbers(&l.1.trim())).collect::<Vec<Vec<u16>>>();
    // println!("{winning_nums:?}");
    let win_nums:Vec<Vec<u16>> = winning_nums.iter().zip(scratch_nums).map(|game| get_win_nums(&game.0, &game.1)).collect();

    for (index, nums) in win_nums.iter().enumerate() {
        for i in 1..=nums.len() {
            if index + i >= scratch_cards_pile.len() {break;}
            scratch_cards_pile[index + i] += scratch_cards_pile[index];
        }
    }

    scratch_cards_pile.iter().fold(0, |acc, el| acc + el)
}

fn remove_gamestring(inputs: &str) -> String {
    inputs.split(':').collect::<Vec<&str>>()[1].to_string()
}

fn divide_lists(number_lists: &str) -> (String, String) {
    let splitted = number_lists.split('|').collect::<Vec<&str>>();

    (splitted[0].to_string(), splitted[1].to_string())
}

fn divide_numbers(number_list: &str) -> Vec<u16> {
    // number_list.split(' ').collect::<Vec<&str>>().iter().inspect(|s| println!("Splitted : {:?}", s), ).filter(|s| !s.trim().is_empty()).map(|num_str| num_str.trim().parse().unwrap()).collect::<Vec<u16>>()
    number_list
        .split_whitespace()
        .filter_map(|num_str| num_str.parse::<u16>().ok())
        .inspect(|s| println!("Splitted : {:?}", s), )
        .collect()
}

fn get_win_nums(winning_nums: &Vec<u16>, scratch_nums: &Vec<u16>) -> Vec<u16> {
    scratch_nums.iter().filter(|s| winning_nums.contains(s)).map(|s| *s).collect::<Vec<u16>>()
}

fn calculate_points(win_nums: &Vec<u16>) -> u32 {
    if win_nums.len() == 0 {0} else {2_u32.pow((win_nums.len() as u32) - 1)}
    
}