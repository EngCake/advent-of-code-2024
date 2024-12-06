fn main() {
    let mut result = 0;

    while let Some(report) = read_report() {
        if is_report_safe(&report) {
            result += 1;
        }
    }

    println!("{result}");
}

fn is_report_safe(report: &Vec<i32>) -> bool {
    return is_report_safe_with_removal(&report, None) || {
        return (0..report.len()).any(|i| is_report_safe_with_removal(&report, Some(i)));
    };
}

fn is_report_safe_with_removal(report: &Vec<i32>, skipped_index: Option<usize>) -> bool {
    return match skipped_index {
        None => is_report_safe_without_removal(&report),
        Some(skipped_index) => {
            let polarity = match skipped_index {
                0 => (report[2] - report[1]).signum(),
                1 => (report[2] - report[0]).signum(),
                _ => (report[1] - report[0]).signum(),
            };

            for i in 1..report.len() {
                if i == skipped_index || i - 1 == skipped_index && skipped_index == 0 {
                    continue;
                }

                let prev_i = if i - 1 == skipped_index { i - 2 } else { i - 1 };
                let delta = report[i] - report[prev_i];
                let abs_delta = delta.abs();
                if delta.signum() != polarity || abs_delta < 1 || abs_delta > 3 {
                    return false;
                }
            }

            return true;
        }
    };
}

fn is_report_safe_without_removal(report: &Vec<i32>) -> bool {
    let polarity = (report[1] - report[0]).signum();
    for window in report.windows(2) {
        let delta = window[1] - window[0];
        let abs_delta = delta.abs();
        if delta.signum() != polarity || abs_delta < 1 || abs_delta > 3 {
            return false;
        }
    }
    return true;
}
fn read_report() -> Option<Vec<i32>> {
    let mut buffer = String::new();
    if std::io::stdin().read_line(&mut buffer).unwrap() == 0 {
        return None;
    }

    let trimmed = buffer.trim();
    if trimmed.is_empty() {
        return None;
    }

    return trimmed
        .split_whitespace()
        .map(|num| num.parse().ok())
        .collect::<Option<Vec<i32>>>();
}
