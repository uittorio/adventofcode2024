use std::fs;

pub fn run() -> i32 {
    let data = fs::read_to_string("inputs/challenge_01a.txt").expect("Cannot find the file");
    let mut first_group = Vec::new();
    let mut second_group = Vec::new();
    data.split("\n")
        .map(|line| line.trim()
            .split(" ")
            .filter(|part| !part.is_empty())
            .map(|part| part.parse::<i32>().unwrap())
            .collect::<Vec<_>>())
        .for_each(|parts| {
            first_group.push(parts[0]);
            second_group.push(parts[1]);
        });

    get_result(first_group, second_group)
}

fn get_result(first: Vec<i32>, second: Vec<i32>) -> i32 {
    let mut result: i32 = 0;
    for value in first.iter() {
        let count = second.iter().filter(|v| v.eq(&value)).count();
        result += value * count as i32;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_numbers_no_differences() {
        let result = get_result(Vec::new(), Vec::new());
        assert_eq!(result, 0);
    }

    #[test]
    fn same_number() {
        let first = vec![1];
        let second = vec![1];
        let result = get_result(first, second);
        assert_eq!(result, 1);
    }

    #[test]
    fn no_numbers_on_left() {
        let first = vec![1];
        let second = vec![];
        let result = get_result(first, second);
        assert_eq!(result, 0);
    }
}