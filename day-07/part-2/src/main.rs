use std::ops::Deref;

use shared::stdin::read_line;

fn main() {
    let mut total = 0;
    while let Some(line) = read_line() {
        let numbers = line.trim().split(' ').collect::<Vec<_>>();
        let result = numbers[0].strip_suffix(':').unwrap().parse().unwrap();
        let numbers = &numbers[1..]
            .iter()
            .map(Deref::deref)
            .map(str::parse)
            .map(Result::unwrap)
            .collect();

        if find_operators(result, &numbers, 1, numbers[0]) {
            total += result;
        }
    }
    println!("{}", total);
}

fn find_operators(result: i64, numbers: &Vec<i64>, index: usize, current: i64) -> bool {
    if index >= numbers.len() {
        return current == result;
    }

    find_operators(result, &numbers, index + 1, current + numbers[index])
        || find_operators(result, &numbers, index + 1, current * numbers[index])
        || find_operators(result, &numbers, index + 1, concat(current, numbers[index]))
}

fn concat(first: i64, second: i64) -> i64 {
    let second_log = (second as f64).log10().floor() + 1f64;
    let base = 10f64.powf(second_log) as i64;
    first * base + second
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concat() {
        assert_eq!(concat(12, 34), 1234);
    }
}
