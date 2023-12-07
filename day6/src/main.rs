use std::fs::read_to_string;
fn main() {
    let inputs: String = read_to_string("input.txt").expect("File with inputs");
    //let sum = part1(&inputs);
    let sum2 = part2(&inputs);
    //println!("Sum 1: {sum}");
    println!("Sum 2: {sum2}");
}


fn part1(inputs: &str) -> u64 {
    let (times, distances) = parse_file(&inputs);

    let possible_distances: Vec<Vec<u64>> = calc_distances(times);
    println!("Distances: {:?}", possible_distances);
    let winning_hold_times = calc_winning_distances(&distances, &possible_distances);

    println!("Winning hold distances: {:?}", winning_hold_times);

    winning_hold_times.iter().map(|win| win.len() as u64).fold(1, |acc, curr| curr * acc)
}

fn calc_distances(times: Vec<u64>) -> Vec<Vec<u64>> {
    let mut distances: Vec<Vec<u64>> = Vec::new();

    distances = times
        .iter()
        .map(|&time| (0..=time)
            .into_iter()
            .map(|velocity| (time - velocity*1)*velocity)
            .collect::<Vec<u64>>()) // Colleziono tutte le possibilità di distanza per il numero di secondi dato
        .collect:: <Vec<Vec<u64>>>(); // Colleziono tutte le distanze per tutti i tempi dati

    distances
}

fn calc_winning_distances(distances: &Vec<u64>, possible_distances: &Vec<Vec<u64>>) ->  Vec<Vec<u64>> {
    distances
        .iter()
        .zip(possible_distances.iter())
        .map(|(&d, p_ds)| 
            p_ds
            .iter()
            .filter(|&&p_d| p_d > d)
            .cloned()
            .collect::<Vec<u64>>()
        ).collect::<Vec<Vec<u64>>>()
}

fn parse_file(inputs: &str) -> (Vec<u64>, Vec<u64>) {
    let vals = inputs
        .lines()
        .map(|line| line
                .split(':')
                .into_iter()
                .skip(1)
                .map(|nums| nums
                    .split_whitespace()
                    .into_iter()
                    .map(|num| num.parse().unwrap())
                    .collect::<Vec<u64>>()
                )
                .collect::<Vec<Vec<u64>>>()
        )
        .flatten()
        .collect::<Vec<Vec<u64>>>();
    
    (vals.get(0).unwrap().clone(), vals.get(1).unwrap().clone())
}

fn parse_file2(inputs: &str) -> (Vec<u64>, Vec<u64>)  {
    let vals = inputs
        .lines()
        .map(|line| line
                .split(':')
                .into_iter()
                .skip(1)
                .map(|nums| nums
                    .split_whitespace()
                    .into_iter()
                    .collect::<Vec<&str>>()
                    .concat()
                    .parse()
                    .unwrap()
                )
                .collect::<Vec<u64>>()
        )
        .collect::<Vec<Vec<u64>>>();
        
        
    (vals.get(0).unwrap().clone(), vals.get(1).unwrap().clone())
}



fn part2(inputs: &str) -> u64 {
    let (times, distances) = parse_file2(&inputs);

    let possible_distances: Vec<Vec<u64>> = calc_distances(times);
    // println!("Distances: {:?}", possible_distances);
    let winning_hold_times = calc_winning_distances(&distances, &possible_distances);

    // println!("Winning hold distances: {:?}", winning_hold_times);

    winning_hold_times.iter().map(|win| win.len() as u64).fold(0, |acc, curr| curr + acc)
}