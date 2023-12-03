use std::fs::read_to_string;

fn main() {
    let inputs: String = read_to_string("input.txt").expect("File with inputs");
    let sum = part1(&inputs);
    let sum2 = part2(&inputs);

    println!("Part 1 Sum: {}", sum);
    println!("Part 2 Sum: {}", sum2);
}

fn part1(file: &str) -> i32 {
    return file.lines()
    .map( |line| line.chars().filter(|c| c.is_numeric()).collect()) // Filtro tutte le linee riportandomi solo i numeri
    .map(|line: String| format!("{}{}", line.chars().next().expect("Expect at least one number"), line.chars().last().expect("Expect at least one number"))) // Prendere solo primo e ultimo numero
    // .inspect(|f| println!("Val: {}", f))
    .fold(0,|acc, curr| curr.parse::<i32>().unwrap() + acc);
}

// fn part2(file: &str) -> i32 {
//     const CIFRE: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
//     file.lines()
//     .map(|line| line.fi)
// }

fn part2(file: &str) -> i32 {
    let mut num_lines = String::new();

    for line in file.lines() {
        const CIFRE: [&str; 10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
        let mut first = String::new();
        let mut last = String::new();
        // println!("Line: {}", line);
        for i in 0..line.len() {
            let partial = &line[i..];
            for (index, cifra) in CIFRE.iter().enumerate() {
                if partial.starts_with(cifra) | partial.starts_with(&index.to_string()) {
                    first = index.to_string();
                    break; 
                }
            };

            if first.len() > 0 {
                break;
            }
        }
        // println!("First: {}", first);

        let rev_line = line.chars().rev().collect::<String>();

        for i in 0..rev_line.len() {
            let partial = &rev_line[i..];
            for (index, cifra) in CIFRE.iter().enumerate() {
                let rev_cifra = cifra.chars().rev().collect::<String>();
                if partial.starts_with(&rev_cifra) | partial.starts_with(&index.to_string()) {
                    last = index.to_string();
                    break; 
                }
            };

            if last.len() > 0 {
                break;
            }
        }

        num_lines.push_str(&format!("{}{}\n",first, last));

        first = "".to_string();
        last = "".to_string();
        
        // println!("============================= LINE ==============================");
        // println!("{}", line)
    }

    num_lines.lines().inspect(|l| println!("{}", l)).count();
    num_lines.lines().fold(0,|acc, curr| curr.parse::<i32>().unwrap() + acc)
}
