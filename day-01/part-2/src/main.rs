use std::{collections::HashMap, io::stdin};

fn main() {
    let mut first_list: Vec<i32> = Vec::new();
    let mut second_list: Vec<i32> = Vec::new();

    while let Some((first, second)) = read_numbers() {
        first_list.push(first);
        second_list.push(second);
    }

    let score = get_similarity(&first_list, &second_list);
    println!("{score}");
}

fn get_similarity(first: &Vec<i32>, second: &Vec<i32>) -> i32 {
    let freq = get_freq(&second);
    return first
        .iter()
        .filter_map(|num| freq.get(num).map(|count| num * count))
        .sum();
}

fn get_freq(list: &Vec<i32>) -> HashMap<i32, i32> {
    let mut freq: HashMap<i32, i32> = HashMap::new();
    for &number in list {
        *freq.entry(number).or_insert(0) += 1;
    }
    return freq;
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
