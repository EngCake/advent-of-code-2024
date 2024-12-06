use std::collections::HashMap;

fn main() {
    let mut first_list: Vec<i32> = Vec::new();
    let mut second_list: Vec<i32> = Vec::new();

    loop {
        let numbers = read_numbers();
        if numbers.is_none() {
            break;
        }

        let (first, second) = numbers.unwrap();
        first_list.push(first);
        second_list.push(second);
    }

    let score = get_similarity(&first_list, &second_list);
    println!("{score}");
}

fn get_similarity(first: &Vec<i32>, second: &Vec<i32>) -> i32 {
    let freq = get_freq(&second);
    let mut score = 0;
    for number in first {
        if freq.contains_key(number) {
            score += number * freq[number];
        }
    }
    return score;
}

fn get_freq(list: &Vec<i32>) -> HashMap<i32, i32> {
    let mut freq: HashMap<i32, i32> = HashMap::new();
    for number in list {
        if !freq.contains_key(number) {
            freq.insert(*number, 0);
        }
        freq.insert(*number, freq.get(number).unwrap() + 1);
    }
    return freq;
}

fn read_numbers() -> Option<(i32, i32)> {
    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer).unwrap();

    if buffer == "\n" {
        return Option::None;
    }

    let mut split_iter = buffer.strip_suffix('\n').unwrap().split_whitespace();
    let first_number = split_iter.next().unwrap();
    let second_number = split_iter.next().unwrap();

    let first_number = first_number.parse::<i32>().unwrap();
    let second_number = second_number.parse::<i32>().unwrap();

    return Option::Some((first_number, second_number));
}
