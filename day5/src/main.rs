use std::{fs::read_to_string, collections::{HashMap, btree_map::Keys}};
fn main() {
    let inputs: String = read_to_string("input.txt").expect("File with inputs");
    // let sum = part1(&inputs);
    let sum2 = part2(&inputs);
    // println!("Sum 1: {sum}");
    println!("Sum 2: {sum2}");
}

fn part1(inputs: &str) -> u64 {
    let (seeds,ordered_keys, maps) = parse_file(inputs);
    let seed_to_location = seeds_to_locations(&seeds, &ordered_keys, &maps);
    let min = seed_to_location[0].last().unwrap();
    let min = seed_to_location.iter().fold(min, |prev,path| path.last().unwrap().min(&prev));
    *min
}

fn parse_file(input_file: &str) -> (Vec<u64>, Vec<String>, HashMap<String, Vec<Vec<u64>>>) {
    let mut maps = HashMap::new();
    let mut seeds = Vec::new() as Vec<u64>;
    let mut ordered_keys = Vec::new() as Vec<String>;
    let mut curr_map = String::new();

    for line in input_file.lines() {
        if line.contains(':') {
            let splitted = line.split(':').collect::<Vec<&str>>();
            let key = splitted[0].trim().to_string();

            if key == "seeds" {
                seeds.append(&mut splitted[1].split_whitespace()
                                       .filter_map(|num| num.parse().ok())
                                       .collect());
            } else {
                curr_map = key.clone();
                ordered_keys.push(key.clone());
                maps.insert(key, Vec::new());
            }
        } else if !line.trim().is_empty() {
            if let Some(values) = maps.get_mut(&curr_map) {
                values.push(Vec::from_iter(&mut line.trim().split_whitespace().filter_map(|num| num.parse::<u64>().ok())))
            }
        }
    }
    // println!("Mappe {:?}", maps);
    (seeds, ordered_keys, maps)
}

fn seeds_to_locations(seeds: &Vec<u64>, ordered_keys: &Vec<String>, maps: &HashMap<String, Vec<Vec<u64>>>) -> Vec<Vec<u64>> {
    let mut seed_to_location_path: Vec<Vec<u64>> = Vec::new();
    seeds.iter().for_each(|&s| seed_to_location_path.push(Vec::from([s])));
    //println!("Seeds {:?}", seed_to_location_path);
    for key in ordered_keys {
        // println!("Key: {}", key);
        seed_to_location_path = seed_to_location_path.iter().map(|path| {
            let mut new_path = path.clone();
            let map = maps.get(key).unwrap();
            let mut found = false;
            for map_el in map {
                // println!("{} {} {}", map_el[0], map_el[1], map_el[2]);
                let new_val = val_to_mapped(path.last().unwrap().clone(), map_el[1], map_el[0], map_el[2]);
                if new_val.is_some()  {
                    found = true;
                    new_path.push(new_val.unwrap());
                }
            }
            if !found {

                new_path.push(path.last().unwrap().clone());
            }

            new_path
        }).collect();
        // println!("{:?}", seed_to_location_path);
    }
    seed_to_location_path
}

fn seed_to_location(seeds: u64, ordered_keys: &Vec<String>, maps: &HashMap<String, Vec<Vec<u64>>>) -> u64 {
    let mut step = seeds.clone();
    for key in ordered_keys {
        let map = maps.get(key).unwrap();

        for map_el in map {
            let new_val = val_to_mapped(step, map_el[1], map_el[0], map_el[2]);
            if new_val.is_some()  {
                step = new_val.unwrap();
                break;
            }
        }
    }
    step
}

fn val_to_mapped(val: u64, start: u64, mapped_start: u64,  lenght: u64) -> Option<u64> {
    
    if val >= start && val < start + lenght {Some(mapped_start + (val - start))} else {None}
}


fn part2(inputs: &str) -> u64 {
    let (seeds,ordered_keys, maps) = parse_file(inputs);
    let mut min_min = u64::MAX;
    let mut iteration = 0;

    for chunk in seeds.chunks(2) {
        iteration += 1;
        println!("Iteration {chunk:?}");
        // let lista = (start..(start + end)).collect::<Vec<u64>>();
        //lista.iter().map(|seed| seed_to_location(&vec![*seed], &ordered_keys, &maps)).map(|seed_to_looc| seed_to_looc[0].last());
        let min = (chunk[0]..(chunk[0] + chunk[1]))
            .map(|y| seed_to_location(y, &ordered_keys, &maps))
            .min()
            .unwrap();

            // for seed in (start..(start + end)) {
            //     min_min = seed_to_location(seed, &ordered_keys, &maps).min(min_min);
            // }
        
        min_min = if min < min_min {min} else {min_min};
    }
    // let seeds = seeds.chunks(2)
    //     .map(|chunk| { 
    //         let start = chunk.get(0).unwrap().clone();
    //         let end = chunk.get(1).unwrap().clone();
    //         let lista = (start..(start + end)).collect::<Vec<u64>>();
    //         // println!("Lista: {lista:?}");
    //         lista
    //     })
    //     .flatten()
    //     .collect::<Vec<u64>>();
    // println!("{}", seeds.len());
    // let seed_to_location = seed_to_location(&seeds, &ordered_keys, &maps);
    // let min = seed_to_location[0].last().unwrap();
    // let min = seed_to_location.iter().fold(min, |prev,path| path.last().unwrap().min(&prev));

    min_min
    
}