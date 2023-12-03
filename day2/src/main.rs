use std::fs::read_to_string;

fn main() {
    let inputs: String = read_to_string("input.txt").expect("File with inputs");
    let sum = part1(&inputs);
    let sum2 = part2(&inputs);

    println!("Somma {}", sum);
    println!("Somma 2 {}", sum2);
}

fn part1(inputs: &str) -> u32 {
    let mut sum: u32 = 0;
    let available_marbles = [12, 13, 14];

    for (index, line) in inputs.lines().enumerate() {
        let mut max_marble = Vec::from([0,0,0]);

        let game: Vec<&str> = line.split(':').collect();
        let id:u32 = game[0].split_ascii_whitespace().collect::<Vec<&str>>().get(1).expect("Game ID must be provided").parse().unwrap();
        let grabs: Vec<&str> = game.get(1).expect("No grabs").split(';').collect();

        for grab in grabs {
            let marbles = grab.split(',').collect::<Vec<&str>>();
            for marble in marbles {
                let color = marble.split_ascii_whitespace().collect::<Vec<&str>>();
                match color.get(1).unwrap() {
                    &"red" => if color.get(0).expect("A number of marbles must be provided").parse::<u32>().unwrap() > max_marble[0] { max_marble[0] = color.get(0).unwrap().parse().unwrap()},
                    &"green" => if color.get(0).expect("A number of marbles must be provided").parse::<u32>().unwrap() > max_marble[1] { max_marble[1] = color.get(0).unwrap().parse().unwrap()},
                    &"blue" => if color.get(0).expect("A number of marbles must be provided").parse::<u32>().unwrap() > max_marble[2] { max_marble[2] = color.get(0).unwrap().parse().unwrap()},
                    _ => println!("altro")
                }
            }
        }

        
        if ((max_marble[0] <= available_marbles[0]) && (max_marble[1] <= available_marbles[1]) && (max_marble[2] <= available_marbles[2]) ) {
            sum += id;
        }
    }

    sum
}

fn part2(inputs: &str) -> u32 {
    let mut sum: u32 = 0;
    let available_marbles = [12, 13, 14];

    for (index, line) in inputs.lines().enumerate() {
        let mut max_marble = Vec::from([0,0,0]);

        let game: Vec<&str> = line.split(':').collect();
        let id:u32 = game[0].split_ascii_whitespace().collect::<Vec<&str>>().get(1).expect("Game ID must be provided").parse().unwrap();
        let grabs: Vec<&str> = game.get(1).expect("No grabs").split(';').collect();

        for grab in grabs {
            let marbles = grab.split(',').collect::<Vec<&str>>();
            for marble in marbles {
                let color = marble.split_ascii_whitespace().collect::<Vec<&str>>();
                match color.get(1).unwrap() {
                    &"red" => if color.get(0).expect("A number of marbles must be provided").parse::<u32>().unwrap() > max_marble[0] { max_marble[0] = color.get(0).unwrap().parse().unwrap()},
                    &"green" => if color.get(0).expect("A number of marbles must be provided").parse::<u32>().unwrap() > max_marble[1] { max_marble[1] = color.get(0).unwrap().parse().unwrap()},
                    &"blue" => if color.get(0).expect("A number of marbles must be provided").parse::<u32>().unwrap() > max_marble[2] { max_marble[2] = color.get(0).unwrap().parse().unwrap()},
                    _ => println!("altro")
                }
            }
        }

        sum += (max_marble[0] * max_marble[1] *max_marble[2]);
    }

    sum
}