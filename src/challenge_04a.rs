use std::fs;
use std::ops::Index;

pub fn run() -> i32 {
    let data = fs::read_to_string("inputs/challenge_04a.txt").expect("Cannot find the file");

    let matrix: Vec<Vec<&str>> = data
        .split("\n")
        .map(|line|
            line.split("").collect::<Vec<_>>())
        .collect::<Vec<_>>();

    do_stuff(matrix)
}

fn do_stuff(matrix: Vec<Vec<&str>>) -> i32 {
    let mut result: i32 = 0;
    for (line_index, line) in matrix.iter().enumerate() {
        for (value_index, value) in line.iter().enumerate() {
            if value == &"X" {
                if search_right(&matrix, line_index, value_index) {
                    result += 1;
                }

                if search_left(&matrix, line_index, value_index) {
                    result += 1;
                }
            }
        }
    }

    result
}

fn search_right(matrix: &Vec<Vec<&str>>, line_index: usize, value_index: usize) -> bool {
    let line = &matrix[line_index];
    if line.iter().count() <= value_index + 3 {
        return false;
    }
    if (line[value_index + 1]) == "M" {
        if line[value_index + 2] == "A" {
            if line[value_index + 3] == "S" {
                return true;
            }
        }
    }

    false
}

fn search_left(matrix: &Vec<Vec<&str>>, line_index: usize, value_index: usize) -> bool {
    let line = &matrix[line_index];

    if value_index < 3 {
        return false;
    }

    if (line[value_index -1]) == "M" {
        if line[value_index - 2] == "A" {
            if line[value_index - 3] == "S" {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_matrix_right() {
        let matrix: Vec<Vec<&str>> = vec![vec!["X", "M", "A", "S"]];
        let result = do_stuff(matrix);
        assert_eq!(result, 1);
    }

    #[test]
    fn simple_matrix_left() {
        let matrix: Vec<Vec<&str>> = vec![vec!["S", "A", "M", "X"]];
        let result = do_stuff(matrix);
        assert_eq!(result, 1);
    }

    #[test]
    fn simple_matrix_right_out_of_range() {
        let matrix: Vec<Vec<&str>> = vec![vec!["X", "M"]];
        let result = do_stuff(matrix);
        assert_eq!(result, 0);

        let matrix: Vec<Vec<&str>> = vec![vec![".", ".", ".", "X", "M", "A"]];
        let result = do_stuff(matrix);
        assert_eq!(result, 0);
    }

    #[test]
    fn final_result() {
        let result = run();
        assert_eq!(result, 103811193);
    }
}