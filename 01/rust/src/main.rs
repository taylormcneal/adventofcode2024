use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, BufRead};

fn main() -> io::Result<()> {
    let filename = "../input";

    let (left_vec, right_vec) = read_input_file(filename)?;

    let part1_solution = calculate_difference(&left_vec, &right_vec);
    println!("Part 1: {}", part1_solution);

    let part2_solution = calculate_similarity(&left_vec, &right_vec);
    println!("Part 2: {}", part2_solution);

    Ok(())
}

fn read_input_file(filename: &str) -> io::Result<(Vec<i32>, Vec<i32>)> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut left_vec = Vec::new();
    let mut right_vec = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let mut parts = line.split_whitespace();

        if let (Some(left), Some(right)) = (parts.next(), parts.next()) {
            if let (Ok(left), Ok(right)) = (left.parse(), right.parse()) {
                left_vec.push(left);
                right_vec.push(right);
            } else {
                eprintln!("Warning: Invalid number format in line: {}", line);
            }
        }
    }

    left_vec.sort();
    right_vec.sort();

    Ok((left_vec, right_vec))
}

fn calculate_difference(left_vec: &[i32], right_vec: &[i32]) -> i32 {
    left_vec.iter()
        .zip(right_vec.iter())
        .map(|(left, right)| (left - right).abs())
        .sum()
}

fn calculate_similarity(left_vec: &[i32], right_vec: &[i32]) -> i32 {
    let mut counter = HashMap::new();
    for &num in right_vec {
        *counter.entry(num).or_insert(0) += 1;
    }

    left_vec.iter()
        .map(|&num| num * counter.get(&num).copied().unwrap_or(0) as i32)
        .sum()
}
