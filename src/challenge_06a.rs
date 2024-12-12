use std::fs;

pub fn run() -> usize {
    let data = fs::read_to_string("inputs/challenge_06a.txt").expect("Cannot find the file");
    let map: Vec<Vec<&str>> = data.split("\n").map(|l| l.split("").collect::<Vec<_>>()).collect::<Vec<_>>();

    find_steps(map)
}

fn find_steps(map: Vec<Vec<&str>>) -> usize {
    let (row, column) = find_start(&map);

    let map_cloned = &mut map.clone();
    let steps = move_arrow(map_cloned, row, column, Facing::UP, 1);

    println!("{:?}", map_cloned);

    steps
}

fn move_arrow(map: &mut Vec<Vec<&str>>, row: usize, column: usize, f: Facing, steps: usize) -> usize {
    match f {
        Facing::UP => {
            if row == 0 {
                return steps;
            }

            let row_up = row - 1;

            let up = map[row_up][column];
            if up == "." {
                map[row_up][column] = "X";
                return move_arrow(map, row_up, column, Facing::UP, steps + 1)
            }

            if up == "X" {
                return move_arrow(map, row_up, column, Facing::UP, steps)
            }

            if up == "#" {
                return move_arrow(map, row, column, Facing::RIGHT, steps)
            }

            steps
        }
        Facing::RIGHT => {
            let column_right = column + 1;

            if map[row].iter().count() == column_right {
                return steps;
            }

            let right = map[row][column_right];
            if right == "." {
                map[row][column_right] = "X";
                return move_arrow(map, row, column_right, Facing::RIGHT, steps + 1)
            }

            if right == "X" {
                return move_arrow(map, row, column_right, Facing::RIGHT, steps)
            }

            if right == "#" {
                return move_arrow(map, row, column, Facing::DOWN, steps)
            }

            steps
        }
        Facing::DOWN => {
            let row_down = row + 1;

            if map.iter().count() == row_down {
                return steps;
            }
            let down = map[row_down][column];
            if down == "." {
                map[row_down][column] = "X";
                return move_arrow(map, row_down, column, Facing::DOWN, steps + 1)
            }

            if down == "X" {
                return move_arrow(map, row_down, column, Facing::DOWN, steps)
            }

            if down == "#" {
                return move_arrow(map, row, column, Facing::LEFT, steps)
            }

            steps
        }
        Facing::LEFT => {
            if column == 0 {
                return steps;
            }

            let column_left = column - 1;

            let left = map[row][column_left];
            if left == "." {
                map[row][column_left] = "X";
                return move_arrow(map, row, column_left, Facing::LEFT, steps + 1)
            }

            if left == "X" {
                return move_arrow(map, row, column_left, Facing::LEFT, steps)
            }

            if left == "#" {
                return move_arrow(map, row, column, Facing::UP, steps)
            }

            steps
        }
    }
}

enum Facing {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

fn find_start(map: &Vec<Vec<&str>>) -> (usize, usize) {
    for (line_index, line) in map.iter().enumerate() {
        for (cell_index, &cell) in line.iter().enumerate() {
            if cell == "^" {
                return (line_index, cell_index)
            }
        }
    }

    (0, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn final_result() {
        let result = run();
        assert_eq!(result, 4973);
    }
}
