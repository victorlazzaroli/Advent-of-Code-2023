use std::{fs::read_to_string, io::stdin};
use regex::Regex;

fn main() {
    let inputs: String = read_to_string("input.txt").expect("File with inputs");
    // let sum = part1(&inputs);
    let sum2 = part2(&inputs);
    // println!("Sum 1: {sum}");
    println!("Sum 2: {sum2}");
}

fn part1(inputs: &str) -> u64 {
    let (path, route_destinations, destination_routes) = parse_file(&inputs);
    let mut count = 0;
    let mut el = route_destinations.iter().position(|dest| *dest == "AAA".to_string()).expect("No starting point");
    let mut end = route_destinations.iter().position(|dest| *dest == "ZZZ".to_string()).expect("No ending point");

    println!("Start: {}, End: {}", el, end);
    while el != end {
        let next_movement = path[count % path.len()];
        // println!("Route ID {el}, route: {:?}, next Move: {next_movement}, calc: {:?}, count: {} ", destination_routes[el], path.len() - 1, count + 1);

        el = if next_movement == 0 {destination_routes[el].0.clone()} else {destination_routes[el].1.clone()};
        count += 1;

        // let mut input = String::new();
        // let n = stdin().read_line(&mut input).unwrap();
    }
    
    count as u64
}

fn part2(inputs: &str) -> u64 {
    let (path, route_destinations, destination_routes) = parse_file(&inputs);
    let mut count: u64 = 0;
    let mut curr_positions:Vec<usize> = Vec::new();
    let mut end_points:Vec<usize> = Vec::new();
    let mut counts:Vec<u64> = Vec::new();
    let mut pat_pos = 0;
    route_destinations.iter().enumerate().for_each(|dest| if dest.1.chars().skip(2).next().unwrap() == 'A' {curr_positions.push(dest.0)} else if dest.1.chars().skip(2).next().unwrap() == 'Z' {end_points.push(dest.0)});

    for (index, &curr_pos) in curr_positions.clone().iter().enumerate() {
        while !end_points.contains(&curr_positions[index]) {
            let next_movement = path[pat_pos];
            curr_positions[index] = if next_movement == 0 {destination_routes[curr_positions[index]].0.clone()} else {destination_routes[curr_positions[index]].1.clone()};
            pat_pos = if pat_pos == path.len() - 1 {0} else {pat_pos + 1};
            count += 1;
        }
        counts.push(count);
        count = 0;

    }
    println!("{counts:?}");
    counts.iter().skip(1).fold(counts[0], |acc, &curr| if acc > curr { mcm(acc, curr)} else {mcm(curr, acc)})
    // while !curr_positions.iter().all(|pos| end_points.contains(pos)) {
    //     let next_movement = path[pat_pos];
    //     // println!("Route ID {el}, route: {:?}, next Move: {next_movement}, calc: {:?}, count: {} ", destination_routes[el], path.len() - 1, count + 1);
    //     curr_positions = curr_positions
    //         .iter()
    //         .map(|curr_pos| if next_movement == 0 {destination_routes[*curr_pos].0.clone()} else {destination_routes[*curr_pos].1.clone()})
    //         .collect();
    //     pat_pos = if pat_pos == path.len() - 1 {0} else {pat_pos + 1};
    //     count += 1;
    //     // println!("Curr_pos {:?}", curr_positions);
    //     // println!("Destination: {:?}\n---------------------------------", end_points);
    //     // let mut input = String::new();
    //     // let n = stdin().read_line(&mut input).unwrap();
    // }

    // println!("Curr_pos {:?}", curr_positions);
    // println!("Destination: {:?}\n---------------------------------", end_points);
    
    // count
    
}

fn parse_file(inputs: &str) -> (Vec<u16>, Vec<String>, Vec<(usize, usize)>) {
    let mut input_lines = inputs.lines();
    let path = input_lines
        .next()
        .map(|line| line
            .chars()
            .map(|c| map_char_path_to_num(&c).unwrap())
            .collect::<Vec<u16>>()
        )
        .unwrap();
    let routes = input_lines
        .skip(1)
        .map(|line| line
            .split('=')
            .collect::<Vec<_>>()
            .into_iter()
            .map(|c| c.trim().to_string())
            .collect::<Vec<_>>()
        )
        .collect::<Vec<Vec<String>>>();
    
    let route_destinations = routes.iter().map(|route| route[0].clone()).collect::<Vec<String>>();
    let destination_routes = routes
        .iter()
        .map(|route| {
            let route_names_tuple = route_destinations_to_route_name(&route[1]).unwrap();
            (route_name_to_route_id(&route_names_tuple.0, &route_destinations).unwrap(), route_name_to_route_id(&route_names_tuple.1, &route_destinations).unwrap())
        })
        .collect::<Vec<_>>();
    println!("Destination routes {:?}", destination_routes);
    println!("Routes Dest{:?}", route_destinations);
    println!("Path: {:?}", path);
    // inputs
    //     .lines()
    //     .map(|line| line.split_whitespace().collect::<Vec<_>>())
    //     .map(|game| {
    //         let cards = game[0].chars().map(|c| Card::new(&c)).collect::<Vec<Card>>();
    //         Hand::new(cards.try_into().expect("AAAAAA non ci sono 5 carte in mano"), game[1].parse().expect("AAAAAAA la puntata non è valida"))
    //     })
    //     .collect::<Vec<Hand>>();
    (path, route_destinations, destination_routes)
}

fn map_char_path_to_num(c: &char) -> Option<u16> {
    match *c {
        'L' => Some(0),
        'R' => Some(1),
        _ => None
    }
}

fn route_destinations_to_route_name(destinations: &str) -> Option<(String, String)> {
    let re = Regex::new(r"\((?P<left>\w+), (?P<right>\w+)\)").unwrap();
    let Some(caps) = re.captures(destinations) else {
        println!("no match!");
        return None;
    };
    Some((caps["left"].to_string(), caps["right"].to_string()))
}

fn route_name_to_route_id(route_name: &str, routes_ids: &Vec<String>) -> Option<usize> {
    routes_ids.iter().position(|id| route_name == *id)
}

fn mcm (a: u64, b: u64) -> u64 {
    a * b / mcd(a, b)
}

fn mcd (a: u64, b: u64) -> u64 {
    if b == 0 { return a }
    mcd(b, a % b) 
}