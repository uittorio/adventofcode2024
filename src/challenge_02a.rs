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
        if is_increasing(report) || is_decreasing(report) {
            result += 1;
        }
    }

    result
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
    fn no_reports() {
        let reports = vec![];
        let result = get_safe_reports(reports);
        assert_eq!(result, 0);
    }

    #[test]
    fn safe_increasing_report() {
        let reports = vec![vec![1, 2, 3, 4, 5]];
        let result = get_safe_reports(reports);
        assert_eq!(result, 1);
    }

    #[test]
    fn safe_decreasing_report() {
        let reports = vec![vec![5, 4, 3, 2, 1]];
        let result = get_safe_reports(reports);
        assert_eq!(result, 1);
    }

    #[test]
    fn safe_decreasing_by2_report() {
        let reports = vec![vec![7, 6, 4, 2, 1]];
        let result = get_safe_reports(reports);
        assert_eq!(result, 1);
    }

    #[test]
    fn safe_increasing_by2_report() {
        let reports = vec![vec![1, 3, 6, 7, 9]];
        let result = get_safe_reports(reports);
        assert_eq!(result, 1);
    }

    #[test]
    fn unsafe_report_increase_decrease() {
        let reports = vec![vec![1, 3, 2, 4, 5]];
        let result = get_safe_reports(reports);
        assert_eq!(result, 0);
    }

    #[test]
    fn unsafe_report_increase_of_5() {
        let reports = vec![vec![1, 2, 7, 8, 9]];
        let result = get_safe_reports(reports);
        assert_eq!(result, 0);
    }

    #[test]
    fn unsafe_report_decrease_of_4() {
        let reports = vec![vec![9, 7, 6, 2, 1]];
        let result = get_safe_reports(reports);
        assert_eq!(result, 0);
    }

    #[test]
    fn no_increase() {
        let reports = vec![vec![8, 6, 4, 4, 1]];
        let result = get_safe_reports(reports);
        assert_eq!(result, 0);
    }
}