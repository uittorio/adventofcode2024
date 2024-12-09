use std::fs;
use std::ops::Index;

pub fn run() -> i32 {
    let data = fs::read_to_string("inputs/challenge_05a.txt").expect("Cannot find the file");


    let split: Vec<&str> = data.split("\n\n").collect::<Vec<_>>();
    let orders = split[0].split("\n").map(|o| {
        let orders = o.split("|").collect::<Vec<_>>();
        return (orders[0].parse::<i32>().unwrap(), orders[1].parse::<i32>().unwrap())
    }).collect::<Vec<_>>();
    let updates: Vec<Vec<i32>> = split[1].split("\n").map(|u|
        u.split(",")
            .map(|p| p.parse::<i32>().unwrap()).collect::<Vec<_>>()
    ).collect::<Vec<_>>();
    let valid_updates = find_valid_updates(&orders, &updates);
    some_middle_valid_updates(valid_updates)
}

fn some_middle_valid_updates(valid_updates: Vec<Vec<i32>>) -> i32 {
    valid_updates.iter().fold(0, |acc, u| {
        let middle_index = (u.iter().count() as f32 / 2.).floor();
        acc + u[middle_index as usize]
    })
}

fn find_valid_updates(orders: &Vec<(i32, i32)>, updates: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut valid_updates: Vec<Vec<i32>> = vec![];

    for update in updates {
        let mut is_valid = true;
        for order in orders {
            let left_index = update.iter().position(|&s| s == order.0);
            let right_index = update.iter().position(|&s| s == order.1);

            if left_index.is_some() && right_index.is_some() {
                if left_index > right_index {
                    is_valid = false
                }
            }
        }

        if is_valid {
            valid_updates.push(update.clone());
        }
    }
    valid_updates
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn final_result() {
        let result = run();
        assert_eq!(result, 2046);
    }

    #[test]
    fn some_valid_and_invalid() {
        let result = find_valid_updates(&vec![(20, 30)], &vec![
            vec![30, 20],
            vec![20, 30],
            vec![31, 20],
        ]);
        assert_eq!(result, vec![
            vec![20, 30],
            vec![31, 20],
        ]);
    }
}
