use std::iter::zip;

fn main() {
    let mut first_list: Vec<i32> = Vec::new();
    let mut second_list: Vec<i32> = Vec::new();

    loop {
        match read_numbers() {
            None => break,
            Some(numbers) => {
                let (first, second) = numbers;
                first_list.push(first);
                second_list.push(second);
            }
        }
    }

    let distance = calculate_distance_between_lists(&first_list, &second_list);
    println!("{distance}");
}

fn calculate_distance_between_lists(list1: &Vec<i32>, list2: &Vec<i32>) -> i32 {
    let mut sorted_list1 = list1.clone();
    let mut sorted_list2 = list2.clone();
    sorted_list1.sort();
    sorted_list2.sort();

    let mut distance = 0;
    for (a, b) in zip(sorted_list1, sorted_list2) {
        distance += i32::abs(a - b);
    }

    return distance;
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
