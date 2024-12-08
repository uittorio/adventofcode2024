use std::fs;

pub fn run() -> i32 {
    let data = fs::read_to_string("inputs/challenge_04a.txt").expect("Cannot find the file");

    let matrix: Vec<Vec<&str>> = data
        .split("\n")
        .map(|line|
            line.split("").collect::<Vec<_>>())
        .collect::<Vec<_>>();

    mas_inx_matches(matrix)
}

fn mas_inx_matches(matrix: Vec<Vec<&str>>) -> i32 {
    let mut result: i32 = 0;
    for (line_index, line) in matrix.iter().enumerate() {
        for (value_index, value) in line.iter().enumerate() {
            if value == &"A" {
                if search_mas(&matrix, line_index, value_index) {
                    result += 1;
                }
            }
        }
    }

    result
}

fn search_mas(matrix: &Vec<Vec<&str>>, line_index: usize, value_index: usize) -> bool {

    if there_is_no_space_up(line_index) {
        return false
    }

    if there_is_no_space_down(matrix, line_index) {
        return false
    }

    if there_is_no_space_left(value_index) {
        return false
    }

    if there_is_no_space_right(matrix, line_index, value_index) {
        return false
    }

    let top_line = &matrix[line_index - 1];
    let bottom_line = &matrix[line_index + 1];

    let top_right_value = top_line[value_index + 1];
    let top_left_value = top_line[value_index - 1];
    let bottom_right_value = bottom_line[value_index + 1];
    let bottom_left_value = bottom_line[value_index -1];

    if top_right_value == "S" && bottom_left_value == "M" && top_left_value == "S" && bottom_right_value == "M" {
        return true
    }

    if top_right_value == "M" && bottom_left_value == "S" && top_left_value == "M" && bottom_right_value == "S" {
        return true
    }

    if top_right_value == "S" && bottom_left_value == "M" && top_left_value == "M" && bottom_right_value == "S" {
        return true
    }

    if top_right_value == "M" && bottom_left_value == "S" && top_left_value == "S" && bottom_right_value == "M" {
        return true
    }

    false
}

fn there_is_no_space_right(matrix: &Vec<Vec<&str>>, line_index: usize, value_index: usize) -> bool {
    let line = &matrix[line_index];
    line.iter().count() - 1 == value_index
}

fn there_is_no_space_down(matrix: &Vec<Vec<&str>>, line_index: usize) -> bool {
    matrix.iter().count() - 1 == line_index
}

fn there_is_no_space_left(value_index: usize) -> bool {
    value_index < 1
}

fn there_is_no_space_up(line_index: usize) -> bool {
    line_index < 1
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_matrix_top_s() {
        let matrix: Vec<Vec<&str>> = vec![
            vec!["S", ".", "S", "."],
            vec![".", "A", ".", "."],
            vec!["M", ".", "M", "."]
        ];
        let result = mas_inx_matches(matrix);
        assert_eq!(result, 1);
    }

    #[test]
    fn simple_matrix_bottom_s() {
        let matrix: Vec<Vec<&str>> = vec![
            vec!["M", ".", "M", "."],
            vec![".", "A", ".", "."],
            vec!["S", ".", "S", "."]
        ];
        let result = mas_inx_matches(matrix);
        assert_eq!(result, 1);
    }

    #[test]
    fn simple_matrix_down_and_up() {
        let matrix: Vec<Vec<&str>> = vec![
            vec!["S", ".", "M", "."],
            vec![".", "A", ".", "."],
            vec!["S", ".", "M", "."]
        ];
        let result = mas_inx_matches(matrix);
        assert_eq!(result, 1);
    }

    #[test]
    fn simple_matrix_down_and_up_2() {
        let matrix: Vec<Vec<&str>> = vec![
            vec!["M", ".", "S", "."],
            vec![".", "A", ".", "."],
            vec!["M", ".", "S", "."]
        ];
        let result = mas_inx_matches(matrix);
        assert_eq!(result, 1);
    }

    #[test]
    fn simple_matrix_no_matching() {
        let matrix: Vec<Vec<&str>> = vec![
            vec!["S", ".", "S", "."],
            vec![".", "A", ".", "."],
            vec!["S", ".", "S", "."]
        ];
        let result = mas_inx_matches(matrix);
        assert_eq!(result, 0);
    }

    #[test]
    fn simple_matrix_no_space() {
        let matrix: Vec<Vec<&str>> = vec![
            vec![".", ".", "A", "."],
            vec!["A", "A", "A", "A"],
            vec![".", ".", "A", "."]
        ];
        let result = mas_inx_matches(matrix);
        assert_eq!(result, 0);
    }

    #[test]
    fn final_result() {
        let result = run();
        assert_eq!(result, 2046);
    }
}
