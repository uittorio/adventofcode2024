use std::fs;

pub fn run() -> usize {
    let data = fs::read_to_string("inputs/challenge_06a.txt").expect("Cannot find the file");
    let map: Vec<Vec<String>> = data.split("\n").map(|l| l.split("").map(|s| s.to_string()).collect::<Vec<_>>()).collect::<Vec<_>>();

    find_steps(map)
}

fn find_steps(map: Vec<Vec<String>>) -> usize {
    let (row, column) = find_start(&map);

    let map_cloned = &mut map.clone();
    let obs = move_arrow(map_cloned, row, column, Facing::UP, &mut vec![]);

    obs.iter().count()
}

fn move_arrow(map: &mut Vec<Vec<String>>, row: usize, column: usize, f: Facing, obstructions: &mut Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    match f {
        Facing::UP => {
            if row == 0 {
                return obstructions.clone();
            }

            let row_up = row - 1;

            let up = &map[row_up][column];

            if has_obstruction(map, Facing::UP, row, column) {
                obstructions.push((row, column));
            }

            if up == "X" {
                return move_arrow(map, row_up, column, Facing::UP, obstructions)
            }

            if up == "." {
                map[row_up][column] = String::from("X");
                return move_arrow(map, row_up, column, Facing::UP, obstructions)
            }

            if up.contains("#") {
                map[row_up][column] = up.to_string() + "U";
                return move_arrow(map, row, column, Facing::RIGHT, obstructions)
            }

            obstructions.clone()
        }
        Facing::RIGHT => {
            let column_right = column + 1;

            if map[row].iter().count() == column_right {
                return obstructions.clone();
            }

            let right = &map[row][column_right];

            if has_obstruction(map, Facing::RIGHT, row, column) {
                obstructions.push((row, column));
            }

            if right == "X" {
                return move_arrow(map, row, column_right, Facing::RIGHT, obstructions)
            }

            if right == "." {
                map[row][column_right] = String::from("X");
                return move_arrow(map, row, column_right, Facing::RIGHT, obstructions)
            }

            if right.contains("#") {
                map[row][column_right] = right.to_string() + "R";
                return move_arrow(map, row, column, Facing::DOWN, obstructions)
            }

            obstructions.clone()
        }
        Facing::DOWN => {
            let row_down = row + 1;

            if map.iter().count() == row_down {
                return obstructions.clone();
            }
            let down = &map[row_down][column];

            if has_obstruction(map, Facing::DOWN, row, column) {
                obstructions.push((row, column));
            }

            if down == "X" {
                return move_arrow(map, row_down, column, Facing::DOWN, obstructions)
            }

            if down == "." {
                map[row_down][column] = String::from("X");
                return move_arrow(map, row_down, column, Facing::DOWN, obstructions)
            }

            if down.contains("#") {
                map[row_down][column] = down.to_string() + "D";
                return move_arrow(map, row, column, Facing::LEFT, obstructions)
            }

            obstructions.clone()
        }
        Facing::LEFT => {
            if column == 0 {
                return obstructions.clone();
            }

            let column_left = column - 1;

            let left = &map[row][column_left];

            if has_obstruction(map, Facing::LEFT, row, column) {
                obstructions.push((row, column));
            }

            if left == "X" {
                return move_arrow(map, row, column_left, Facing::LEFT, obstructions)
            }

            if left == "." {
                map[row][column_left] = String::from("X");
                return move_arrow(map, row, column_left, Facing::LEFT, obstructions)
            }

            if left.contains("#") {
                map[row][column_left] = left.to_string() + "L";
                return move_arrow(map, row, column, Facing::UP, obstructions)
            }

            obstructions.clone()
        }
    }
}

fn has_obstruction(map: &Vec<Vec<String>>, facing: Facing, row: usize, column: usize) -> bool {
    match facing {
        Facing::UP => {
            for index in column + 1.. map[row].iter().count() - 1 {
                if map[row][index].contains("R") {
                    return true;
                }
            }
            false
        }
        Facing::RIGHT => {
            for index in row + 1..map.iter().count() - 1 {
                if map[index][column].contains("D") {
                    return true;
                }
            }

            false
        }
        Facing::DOWN => {
            for index in 0.. column - 1 {
                if map[row][index].contains("L") {
                    return true;
                }
            }
            false
        }
        Facing::LEFT => {
            for index in 0.. row - 1 {
                if map[index][column].contains("U") || map[row][index].contains("^") {
                    return true;
                }
            }
            false
        }
    }
}

enum Facing {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

fn find_start(map: &Vec<Vec<String>>) -> (usize, usize) {
    for (line_index, line) in map.iter().enumerate() {
        for (cell_index, cell) in line.iter().enumerate() {
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
