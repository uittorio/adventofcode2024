use std::fs;

pub fn run() -> i32 {
    let data = fs::read_to_string("inputs/challenge_02a.txt").expect("Cannot find the file");
    let reports: Vec<Vec<i32>> = data
        .split("\n")
        .map(|line|
            line.trim()
                .split(" ")
                .filter(|part| !part.is_empty())
                .map(|part| part.parse::<i32>().unwrap()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    get_safe_reports(reports)
}

fn get_safe_reports(reports: Vec<Vec<i32>>) -> i32 {
    let mut result: i32 = 0;
    for report in reports.iter() {
        if is_safe_rec(report, 0) {
            result += 1;
        }
    }

    result
}

fn is_safe(report: &Vec<i32>) -> bool {
    is_increasing(report) || is_decreasing(report)
}

fn is_safe_rec(report: &Vec<i32>, index_to_remove: usize) -> bool {
    if is_safe(&report) {
        return true;
    }

    if index_to_remove >= report.len() {
        return false;
    }

    let mut report_without_index = report.clone();
    report_without_index.remove(index_to_remove);

    if is_safe(&report_without_index) {
        return true;
    }

    is_safe_rec(&report, index_to_remove + 1)
}

fn is_increasing(report: &Vec<i32>) -> bool {
    let mut result: bool = false;
    let mut previous_value: i32 = report.first().unwrap().clone();
    for level in report.iter().skip(1) {
        let diff = level - &previous_value;
        if diff <= 3 && diff > 0 {
            result = true;
        } else {
            result = false;
            break;
        }

        previous_value = level.clone();
    }

    result
}

fn is_decreasing(report: &Vec<i32>) -> bool {
    let mut result: bool = false;
    let mut previous_value: i32 = report.first().unwrap().clone();
    for level in report.iter().skip(1) {
        let diff = &previous_value - level;
        if diff <= 3 && diff > 0 {
            result = true;
        } else {
            result = false;
            break;
        }

        previous_value = level.clone();
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn safe_on_removing_second_index() {
        let reports = vec![vec![1, 3, 2, 4, 5]];
        let result = get_safe_reports(reports);
        assert_eq!(result, 1);
    }

    #[test]
    fn safe_on_removing_first_index() {
        let reports = vec![vec![1, 4, 3, 2, 1]];
        let result = get_safe_reports(reports);
        assert_eq!(result, 1);
    }

    #[test]
    fn unsafe_always() {
        let reports = vec![vec![1, 2, 7, 8, 9]];
        let result = get_safe_reports(reports);
        assert_eq!(result, 0);
    }

    #[test]
    fn final_result() {
        let result = run();
        assert_eq!(result, 665);
    }
}