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

    first_group.sort();
    second_group.sort();

    let mut result:i32 = 0;
    for (index, value) in first_group.iter().enumerate() {
        let diff = (value - second_group[index]).abs();
        result += diff;
    }

    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn final_result() {
        let result = run();
        assert_eq!(result, 2970687);
    }
}