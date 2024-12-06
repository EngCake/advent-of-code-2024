use std::{io::stdin, iter::zip};

fn main() {
    let mut first_list: Vec<i32> = Vec::new();
    let mut second_list: Vec<i32> = Vec::new();

    while let Some((first, second)) = read_numbers() {
        first_list.push(first);
        second_list.push(second);
    }

    let distance = calculate_distance_between_lists(&first_list, &second_list);
    println!("{distance}");
}

fn calculate_distance_between_lists(list1: &Vec<i32>, list2: &Vec<i32>) -> i32 {
    let mut sorted_list1 = list1.clone();
    let mut sorted_list2 = list2.clone();
    sorted_list1.sort();
    sorted_list2.sort();

    return zip(sorted_list1, sorted_list2)
        .map(|(first, second)| (first - second).abs())
        .sum();
}

fn read_numbers() -> Option<(i32, i32)> {
    let mut buffer = String::new();
    if stdin().read_line(&mut buffer).unwrap() == 0 {
        return None;
    }

    if buffer == "\n" {
        return Option::None;
    }

    let numbers = buffer
        .trim()
        .split_whitespace()
        .filter_map(|num| num.parse().ok())
        .collect::<Vec<i32>>();

    return if numbers.len() == 2 {
        Some((numbers[0], numbers[1]))
    } else {
        None
    };
}
