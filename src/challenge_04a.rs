use std::fs;

pub fn run() -> i32 {
    let data = fs::read_to_string("inputs/challenge_04a.txt").expect("Cannot find the file");

    let matrix: Vec<Vec<&str>> = data
        .split("\n")
        .map(|line|
            line.split("").collect::<Vec<_>>())
        .collect::<Vec<_>>();

    xmas_matches(matrix)
}

fn xmas_matches(matrix: Vec<Vec<&str>>) -> i32 {
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

                if search_down(&matrix, line_index, value_index) {
                    result += 1;
                }

                if search_up(&matrix, line_index, value_index) {
                    result += 1;
                }

                if search_diagonal_up_right(&matrix, line_index, value_index) {
                    result += 1;
                }

                if search_diagonal_up_left(&matrix, line_index, value_index) {
                    result += 1;
                }

                if search_diagonal_down_right(&matrix, line_index, value_index) {
                    result += 1;
                }

                if search_diagonal_down_left(&matrix, line_index, value_index) {
                    result += 1;
                }
            }
        }
    }

    result
}

fn there_is_no_space_right(matrix: &Vec<Vec<&str>>, line_index: usize, value_index: usize) -> bool {
    let line = &matrix[line_index];
    line.iter().count() <= value_index + 3
}

fn there_is_no_space_down(matrix: &Vec<Vec<&str>>, line_index: usize) -> bool {
    matrix.iter().count() <= (line_index + 3)
}

fn there_is_no_space_left(value_index: usize) -> bool {
    value_index < 3
}

fn there_is_no_space_up(line_index: usize) -> bool {
    line_index < 3
}

fn matching_mas(value1: &str, value2: &str, value3: &str) -> bool {
    value1 == "M" && value2 == "A" && value3 == "S"
}

fn search_right(matrix: &Vec<Vec<&str>>, line_index: usize, value_index: usize) -> bool {
    if there_is_no_space_right(matrix, line_index, value_index) {
        return false;
    }

    let line = &matrix[line_index];

    matching_mas(line[value_index + 1], line[value_index + 2], line[value_index + 3])
}

fn search_left(matrix: &Vec<Vec<&str>>, line_index: usize, value_index: usize) -> bool {
    let line = &matrix[line_index];

    if there_is_no_space_left(value_index) {
        return false
    }

    matching_mas(line[value_index - 1], line[value_index - 2], line[value_index - 3])
}

fn search_down(matrix: &Vec<Vec<&str>>, line_index: usize, value_index: usize) -> bool {
    if there_is_no_space_down(matrix, line_index) {
        return false;
    }

    let line_1 = &matrix[line_index + 1];
    let line_2 = &matrix[line_index + 2];
    let line_3 = &matrix[line_index + 3];
    matching_mas(line_1[value_index], line_2[value_index], line_3[value_index])
}

fn search_up(matrix: &Vec<Vec<&str>>, line_index: usize, value_index: usize) -> bool {
    if there_is_no_space_up(line_index) {
        return false;
    }

    let line_1 = &matrix[line_index - 1];
    let line_2 = &matrix[line_index - 2];
    let line_3 = &matrix[line_index - 3];

    matching_mas(line_1[value_index], line_2[value_index], line_3[value_index])
}

fn search_diagonal_up_right(matrix: &Vec<Vec<&str>>, line_index: usize, value_index: usize) -> bool {
    if there_is_no_space_up(line_index) || there_is_no_space_right(matrix, line_index, value_index) {
        return false;
    }

    let line_1 = &matrix[line_index - 1];
    let line_1_value = line_1[value_index + 1];

    let line_2 = &matrix[line_index - 2];
    let line_2_value = line_2[value_index + 2];

    let line_3 = &matrix[line_index - 3];
    let line_3_value = line_3[value_index + 3];

    matching_mas(line_1_value, line_2_value, line_3_value)
}

fn search_diagonal_up_left(matrix: &Vec<Vec<&str>>, line_index: usize, value_index: usize) -> bool {
    if there_is_no_space_up(line_index) || there_is_no_space_left(value_index) {
        return false;
    }

    let line_1 = &matrix[line_index - 1];
    let line_1_value = line_1[value_index - 1];

    let line_2 = &matrix[line_index - 2];
    let line_2_value = line_2[value_index - 2];

    let line_3 = &matrix[line_index - 3];
    let line_3_value = line_3[value_index - 3];

    matching_mas(line_1_value, line_2_value, line_3_value)
}

fn search_diagonal_down_right(matrix: &Vec<Vec<&str>>, line_index: usize, value_index: usize) -> bool {
    if there_is_no_space_down(matrix, line_index) || there_is_no_space_right(matrix, line_index, value_index) {
        return false;
    }

    let line_1 = &matrix[line_index + 1];
    let line_1_value = line_1[value_index + 1];

    let line_2 = &matrix[line_index + 2];
    let line_2_value = line_2[value_index + 2];

    let line_3 = &matrix[line_index + 3];
    let line_3_value = line_3[value_index + 3];

    matching_mas(line_1_value, line_2_value, line_3_value)
}

fn search_diagonal_down_left(matrix: &Vec<Vec<&str>>, line_index: usize, value_index: usize) -> bool {
    if there_is_no_space_down(matrix, line_index) || there_is_no_space_left(value_index) {
        return false;
    }

    let line_1 = &matrix[line_index + 1];
    let line_1_value = line_1[value_index - 1];

    let line_2 = &matrix[line_index + 2];
    let line_2_value = line_2[value_index - 2];

    let line_3 = &matrix[line_index + 3];
    let line_3_value = line_3[value_index - 3];

    matching_mas(line_1_value, line_2_value, line_3_value)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_matrix_right() {
        let matrix: Vec<Vec<&str>> = vec![vec!["X", "M", "A", "S"]];
        let result = xmas_matches(matrix);
        assert_eq!(result, 1);
    }

    #[test]
    fn simple_matrix_left() {
        let matrix: Vec<Vec<&str>> = vec![vec!["S", "A", "M", "X"]];
        let result = xmas_matches(matrix);
        assert_eq!(result, 1);
    }

    #[test]
    fn simple_matrix_right_out_of_range() {
        let matrix: Vec<Vec<&str>> = vec![vec!["X", "M"]];
        let result = xmas_matches(matrix);
        assert_eq!(result, 0);

        let matrix: Vec<Vec<&str>> = vec![vec![".", ".", ".", "X", "M", "A"]];
        let result = xmas_matches(matrix);
        assert_eq!(result, 0);
    }

    #[test]
    fn simple_matrix_down() {
        let matrix: Vec<Vec<&str>> = vec![
            vec!["S", "A", "X", "X"],
            vec!["S", "A", "M", "A"],
            vec!["S", "A", "A", "X"],
            vec!["A", "A", "S", "X"]
        ];
        let result = xmas_matches(matrix);
        assert_eq!(result, 1);
    }

    #[test]
    fn simple_matrix_down_out_of_range() {
        let matrix: Vec<Vec<&str>> = vec![
            vec!["S", "A", "X", "X"],
            vec!["S", "A", "A", "X"],
            vec!["S", "A", "A", "M"],
            vec!["A", "A", "A", "A"]
        ];
        let result = xmas_matches(matrix);
        assert_eq!(result, 0);
    }

    #[test]
    fn simple_matrix_up() {
        let matrix: Vec<Vec<&str>> = vec![
            vec!["S", "A", "S", "A"],
            vec!["S", "A", "A", "B"],
            vec!["S", "A", "M", "S"],
            vec!["A", "A", "X", "B"]
        ];
        let result = xmas_matches(matrix);
        assert_eq!(result, 1);
    }

    #[test]
    fn simple_matrix_up_out_of_range() {
        let matrix: Vec<Vec<&str>> = vec![
            vec!["S", "A", "A", "A"],
            vec!["S", "A", "M", "B"],
            vec!["S", "A", "X", "S"],
            vec!["A", "A", "M", "B"]
        ];
        let result = xmas_matches(matrix);
        assert_eq!(result, 0);
    }

    #[test]
    fn simple_matrix_diagonal_up_right() {
        let matrix: Vec<Vec<&str>> = vec![
            vec![".", ".", ".", "S"],
            vec![".", ".", "A", "."],
            vec![".", "M", ".", "."],
            vec!["X", ".", ".", "."]
        ];
        let result = xmas_matches(matrix);
        assert_eq!(result, 1);
    }

    #[test]
    fn simple_matrix_diagonal_up_left() {
        let matrix: Vec<Vec<&str>> = vec![
            vec!["S", ".", ".", "."],
            vec![".", "A", ".", "."],
            vec![".", ".", "M", "."],
            vec![".", ".", ".", "X"]
        ];
        let result = xmas_matches(matrix);
        assert_eq!(result, 1);
    }

    #[test]
    fn simple_matrix_diagonal_down_right() {
        let matrix: Vec<Vec<&str>> = vec![
            vec![".", ".", ".", "X"],
            vec![".", ".", "M", "."],
            vec![".", "A", ".", "."],
            vec!["S", ".", ".", "."]
        ];
        let result = xmas_matches(matrix);
        assert_eq!(result, 1);
    }

    #[test]
    fn final_result() {
        let result = run();
        assert_eq!(result, 2718);
    }
}