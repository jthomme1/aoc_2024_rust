use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;

fn main() {
    let (first_list, second_list) = read_input();
    println!(
        "{}",
        compute_sorted_difference_for_part_one(first_list.clone(), second_list.clone())
    );
    println!(
        "{}",
        compute_similarity_score_for_part_two(first_list, second_list)
    );
}

fn compute_sorted_difference_for_part_one(
    mut first_list: Vec<i32>,
    mut second_list: Vec<i32>,
) -> i32 {
    first_list.sort();
    second_list.sort();
    zip(first_list, second_list)
        .map(|(first, second)| (first - second).abs())
        .sum()
}

fn compute_similarity_score_for_part_two(first_list: Vec<i32>, second_list: Vec<i32>) -> i32 {
    first_list
        .into_iter()
        .map(|x| x * (second_list.iter().filter(|y| **y == x).count() as i32))
        .sum()
}

fn read_input() -> (Vec<i32>, Vec<i32>) {
    let input_path = "input";
    let f = File::open(input_path).unwrap();

    let reader = BufReader::new(f);

    let mut first_list = Vec::new();
    let mut second_list = Vec::new();

    for line in reader.lines() {
        let line_result = line.unwrap();
        let numbers_in_line: Vec<&str> = line_result.split_whitespace().collect();
        assert_eq!(numbers_in_line.len(), 2);
        first_list.push(numbers_in_line[0].parse::<i32>().unwrap());
        second_list.push(numbers_in_line[1].parse::<i32>().unwrap());
    }
    (first_list, second_list)
}
