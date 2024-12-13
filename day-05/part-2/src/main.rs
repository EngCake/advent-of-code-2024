use std::{
    collections::{HashMap, HashSet},
    io::stdin,
    str::FromStr,
};

fn main() {
    let mut precedence = Precedence::new();

    while let Some(line) = read_line() {
        let split_text = line.split('|').map(str::trim).collect::<Vec<_>>();
        let from = split_text[0].parse::<i32>().unwrap();
        let to = split_text[1].parse::<i32>().unwrap();

        precedence.add_rule(from, to);
    }

    let mut result = 0;

    while let Some(line) = read_line() {
        let numbers: Vec<i32> = line
            .split(',')
            .map(str::trim)
            .map(FromStr::from_str)
            .map(Result::unwrap)
            .collect();

        if !check_updates_order(&numbers, &precedence) {
            let mut ordered_numbers = Vec::new();
            fix_updates_order(numbers, &precedence, &mut ordered_numbers);
            result += ordered_numbers[ordered_numbers.len() / 2];
        }
    }

    println!("{}", result);
}

fn fix_updates_order(numbers: Vec<i32>, precedence: &Precedence, ordered_numbers: &mut Vec<i32>) {
    for number in numbers {
        let mut added = false;
        for j in 0..ordered_numbers.len() {
            if precedence.check_order(number, ordered_numbers[j]) {
                ordered_numbers.insert(j, number);
                added = true;
                break;
            }
        }
        if !added {
            ordered_numbers.insert(ordered_numbers.len(), number);
        }
    }
}

fn check_updates_order(numbers: &Vec<i32>, precedence: &Precedence) -> bool {
    let mut correct_order = true;
    for i in 0..numbers.len() {
        for j in i + 1..numbers.len() {
            if !precedence.check_order(numbers[i], numbers[j]) {
                correct_order = false;
                break;
            }
        }
    }
    correct_order
}

#[derive(Debug)]
struct Precedence {
    precedences: HashMap<i32, HashSet<i32>>,
}

impl Precedence {
    fn new() -> Self {
        Self {
            precedences: HashMap::new(),
        }
    }

    fn add_rule(&mut self, first: i32, second: i32) {
        let entry = self.precedences.entry(first);
        entry.or_insert(HashSet::new()).insert(second);
    }

    fn check_order(&self, first: i32, second: i32) -> bool {
        if let Some(first_precedences) = self.precedences.get(&first) {
            first_precedences.contains(&second)
        } else if let Some(second_precedences) = self.precedences.get(&second) {
            !second_precedences.contains(&first)
        } else {
            true
        }
    }
}

fn read_line() -> Option<String> {
    let mut buffer = String::new();
    if stdin().read_line(&mut buffer).ok()? == 0 {
        return None;
    }
    if buffer == "\n" {
        return None;
    }
    return Some(buffer);
}
