use std::fs;
use regex::{Captures, Match, Matches, Regex};

pub fn run() -> i32 {
    let data = fs::read_to_string("inputs/challenge_03a.txt").expect("Cannot find the file");

    let regex = Regex::new(r"mul\((\d+,\d+)\)").unwrap();

    let to_multiply: Vec<(i32, i32)> = regex.captures_iter(&data).map(|c| {
        let split = c.get(1)
            .unwrap()
            .as_str()
            .split(",")
            .map(|v| v.parse::<i32>().unwrap()).collect::<Vec<_>>();

        let x = (split[0], split[1]);
        return x;
    }).collect::<Vec<_>>();


    multiply_sum(to_multiply)
}

fn multiply_sum(data: Vec<(i32, i32)>) -> i32 {
    data.iter().map(|m| m.0 * m.1).reduce(|x,y| x + y).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn safe_on_removing_second_index() {
        let to_multiply = vec![(10, 20), (40, 1)];
        let result = multiply_sum(to_multiply);
        assert_eq!(result, 240);
    }
}