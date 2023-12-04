use std::fs::read_to_string;
use std::io;

fn main() {
    let inputs: String = read_to_string("input.txt").expect("File with inputs");
    // let sum = part1(&inputs);
    let sum2 = part2(&inputs);
    // println!("Sum 1: {sum}");
    println!("Sum 2: {sum2}");
}


fn part1(inputs: &str) -> u32 {
    let lines = inputs.split_terminator('\n').collect::<Vec<&str>>();
    let mut sum = 0;
    for index in 0..lines.len() {
        let prev = if index > 0 {lines.get(index - 1)} else {None};
        let curr = lines.get(index).expect("Expected Line"); 
        let next = if index < (lines.len() - 1) {lines.get(index + 1)} else {None};
        sum += collect_validnums(prev, curr, next);
        // let mut nome = String::new();
        // println!("prev: {prev:?}\ncurr: {curr:?}\nnext: {next:?}");
        // io::stdin().read_line(&mut nome).expect("fd");
    }
    sum
}

fn collect_validnums(prev: Option<&&str>, curr: &&str, next: Option<&&str> ) -> u32 {
    let mut validnums = Vec::<u32>::new();
    let mut num: String = String::new();
    let mut num_indeces = Vec::<usize>::new();
    let mut sum = 0;
    
    for (index, char) in curr.char_indices() {
        // println!("index: {index:?}\nchar: {char:?}\nvalidnums: {validnums:?}");
        if char.is_numeric() {
            num_indeces.push(index);
            num.push(char);
            if index < curr.len() - 1 {continue};
        }

        if num_indeces.len() > 0 {
            
            let start = num_indeces.first().unwrap();
            let end = num_indeces.last().unwrap();
            let start = if start.eq(&0) {0} else {start - 1}; // Calcola inizio della finestra di controllo
            let end = (end + 1).clamp(0, curr.len() - 1); // Calcola fine della finestra di controllo

            // Es. finestra
            // ...#.
            // .333.
            // .....
            // Casi particolari sono i bordi del file e delle righe

            let mut compare_string = String::new();
            compare_string = compare_string + 
                            &curr[start..start+1] + 
                            &curr[end..end+1] + 
                            if prev.is_some() {&prev.unwrap()[start..=end]} else {""} + 
                            if next.is_some() {&next.unwrap()[start..=end]} else {""}; // Compone la finestra di controllo
            // Controlla che nella finestra di controllo ci sia almeno un carattere diverso da . e da un numero
            if (compare_string.chars().any(|c| c != '.' && !c.is_ascii_digit())) {
                validnums.push(num.parse().unwrap());
                sum += num.parse::<u32>().unwrap();
            }
        }

        num_indeces.clear();
        num.clear();
    }

    sum
}


fn part2(inputs: &str) -> u32 {
    let inputs_to_chars_matrix = inputs.split('\n').map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let mut star_pos: Vec<(usize, usize)> = Vec::new();
    let mut valid_num_pos: Vec<(usize, usize)> = Vec::new();
    let mut valid_nums: Vec<u32> = Vec::new();

    for (rowIdx, row) in inputs_to_chars_matrix.iter().enumerate() {
        // println!("{row:?}");
        for (cIdx, c) in row.iter().enumerate() {
            if isStar(&c) {
                star_pos.push((rowIdx, cIdx))
            }
        }
    }

    for star_idx in &star_pos {
        let mut neighbours = numNeighbors(&inputs_to_chars_matrix, star_idx);

        
        if neighbours.len() != 2 {
            continue;
        }

        valid_num_pos.append(&mut neighbours);
        // io::stdin().read_line(&mut nome).expect("fd");
        
    }

    valid_nums = valid_num_pos.iter().map(|f| extract_num(&inputs_to_chars_matrix, f) ).collect();
    println!("{valid_nums:?}");
    let result = valid_nums.chunks(2).inspect(|chunk| println!("Chunk {chunk:?}")).map(|chunk| chunk[0] * chunk[1]).inspect(|step| println!("Step {step:?}")).reduce(|acc, curr| {println!("{acc} : {curr}"); acc + curr});


    // let mut nome = String::new();
    // println!("neighbours: {valid_num_pos:?}");


    // println!("{star_pos:?}");

    result.unwrap()
}

fn isStar(c: &char) -> bool {
    c.eq(&'*')
}

fn numNeighbors(inputs_to_chars_matrix: &Vec<Vec<char>>, pos: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbors: Vec<(usize, usize)> = Vec::new();
    let row_len = inputs_to_chars_matrix[0].len() - 1;
    // println!("{}", inputs_to_chars_matrix[pos.0][pos.1]);
    let radix = 10;
    /* prev row */
    if pos.0 > 0 {
        let start = (pos.0 - 1, if pos.1 == 0 {0} else {pos.1 - 1});
        let end = (pos.0 - 1, if pos.1 == row_len {row_len} else {pos.1 + 1});
        let len_control = end.1 - start.1;

        if len_control == 2 && inputs_to_chars_matrix[start.0][start.1].is_digit(radix) && inputs_to_chars_matrix[end.0][end.1].is_digit(radix) && !inputs_to_chars_matrix[start.0][pos.1].is_digit(radix) {
            neighbors.push(start);
            neighbors.push(end);
        } else if len_control == 2 && inputs_to_chars_matrix[start.0][start.1].is_digit(radix) && inputs_to_chars_matrix[end.0][end.1].is_digit(radix) && inputs_to_chars_matrix[start.0][pos.1].is_digit(radix) {
            neighbors.push(start);
        } else if len_control == 2 && inputs_to_chars_matrix[start.0][pos.1].is_digit(radix) {
            neighbors.push((start.0, pos.1));
        } else if inputs_to_chars_matrix[start.0][start.1].is_digit(radix) {
            neighbors.push(start);
        } else if inputs_to_chars_matrix[end.0][end.1].is_digit(radix) {
            neighbors.push(end);
        }
    }

    //before and after
    if pos.1 > 0 && inputs_to_chars_matrix[pos.0][pos.1 - 1].is_digit(radix) {
        let before = (pos.0, pos.1 - 1);
        neighbors.push(before);
    }
    if pos.1 < row_len && inputs_to_chars_matrix[pos.0][pos.1 + 1].is_digit(radix) {
        let after = (pos.0, pos.1 + 1);
        neighbors.push(after);
    }

    // next row
    if pos.0 < inputs_to_chars_matrix.len() - 1 {
        let start = (pos.0 + 1, if pos.1 == 0 {0} else {pos.1 - 1});
        let end = (pos.0 + 1, if pos.1 == row_len {row_len} else {pos.1 + 1});
        let len_control = end.1 - start.1;

        if len_control == 2 && inputs_to_chars_matrix[start.0][start.1].is_digit(radix) && inputs_to_chars_matrix[end.0][end.1].is_digit(radix) && !inputs_to_chars_matrix[start.0][pos.1].is_digit(radix) {
            neighbors.push(start);
            neighbors.push(end);
        } else if len_control == 2 && inputs_to_chars_matrix[start.0][start.1].is_digit(radix) && inputs_to_chars_matrix[end.0][end.1].is_digit(radix) && inputs_to_chars_matrix[start.0][pos.1].is_digit(radix) {
            neighbors.push(start);
        } else if len_control == 2 && inputs_to_chars_matrix[start.0][pos.1].is_digit(radix) {
            neighbors.push((start.0, pos.1));
        } else if inputs_to_chars_matrix[start.0][start.1].is_digit(radix) {
            neighbors.push(start);
        } else if inputs_to_chars_matrix[end.0][end.1].is_digit(radix) {
            neighbors.push(end);
        }
    }

    neighbors
}




// fn collect_validnums2(prev: Option<&&str>, curr: &&str, next: Option<&&str> ) -> u32 {
//     let mut validnums = Vec::<u32>::new();
//     let mut num: String = String::new();
//     let mut num_indeces = Vec::<usize>::new();
//     let mut sum = 0;

//     sum.end
    
//     for (index, char) in curr.char_indices() {
//         // println!("index: {index:?}\nchar: {char:?}\nvalidnums: {validnums:?}");
//         if char != '*' {
//             continue;
//         }
        
//         let start = if index.eq(&0) {0} else {index - 1}; // Calcola inizio della finestra di controllo
//         let end = (index + 1).clamp(0, curr.len() - 1); // Calcola fine della finestra di controllo

        
//     }

//     sum
// }


fn extract_num(inputs_to_chars_matrix: &Vec<Vec<char>>, pos: &(usize, usize)) -> u32 {
    let mut digits = String::new();
    let stringa_as_bytes = &inputs_to_chars_matrix[pos.0];
    let mut c: char = stringa_as_bytes[pos.1];
    let mut delta = 0;
    let mut start = pos.1;
    // println!("Riga {stringa_as_bytes:?}");
    while c.is_ascii_digit() {
        delta += 1;
        
        if pos.1 < delta {break}
        
        c = stringa_as_bytes[pos.1 - delta];

    }
    start = (pos.1 + 1) - delta;
    delta = 0;

    loop {
        c = stringa_as_bytes[start + delta];
        if !c.is_ascii_digit() {break;}
        digits.push(c);
        delta += 1;
    }

    // let mut nome = String::new();
    // println!("digits: {digits:?}");
    // io::stdin().read_line(&mut nome).expect("fd");
    
    digits.parse().expect("Parse to num error")

}