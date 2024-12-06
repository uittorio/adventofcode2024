use regex::Regex;
use std::fs;

pub fn run() -> i32 {
    let data = fs::read_to_string("inputs/challenge_03a.txt").expect("Cannot find the file");

    let data_do: Vec<&str> = data
        .split("do()")
        .map(|d| d.split("don't()").take(1).collect::<Vec<_>>())
        .flatten()
        .collect::<Vec<_>>();

    let mut all_to_multiply = vec![];

    for d in data_do {
        let to_multiply = Regex::new(r"mul\((\d+,\d+)\)").unwrap().captures_iter(d).map(|c| {
            let split = c.get(1)
                .unwrap()
                .as_str()
                .split(",")
                .map(|v| v.parse::<i32>().unwrap()).collect::<Vec<_>>();

            let x = (split[0], split[1]);
            return x;
        });
        for x in to_multiply {
            all_to_multiply.push(x)
        }
    }

    multiply_sum(all_to_multiply)
}

fn multiply_sum(data: Vec<(i32, i32)>) -> i32 {
    data.iter().map(|m| m.0 * m.1).reduce(|x,y| x + y).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn final_result() {
        let result = run();
        assert_eq!(result, 103811193);
    }
}