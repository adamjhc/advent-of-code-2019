// I can see two possible solutions to this problem:
// 1. The brute force way - calculate all the positions each line encounters and
//    then find which positions are encountered by both lines giving you the
//    intersections - very slow
// 2. The "smarter" way - calculate the start and end points of each line and
//    find the intersection points by finding where each line intersects with
//    another - complex math
use utils;

fn main() {
    println!("{}", part_1());
    println!("{}", part_2());
}

#[derive(Clone, Copy, PartialEq, Debug, Eq)]
struct Pos {
    x: isize,
    y: isize,
}

fn part_1() -> isize {
    let paths: Vec<Vec<String>> = utils::read_string("./input.txt")
        .split("\n")
        .map(|a| a.split(",").map(|a| a.to_string()).collect())
        .collect();

    let path_1 = generate_path(&paths[0]);
    let path_2 = generate_path(&paths[1]);

    path_1
        .iter()
        .filter(|&pos| path_2.contains(pos))
        .map(|pos| calculate_manhatten_distance(pos))
        .min()
        .unwrap()
}

fn part_2() -> usize {
    let paths: Vec<Vec<String>> = utils::read_string("./input.txt")
        .split("\n")
        .map(|a| a.split(",").map(|a| a.to_string()).collect())
        .collect();

    let path_1 = generate_path(&paths[0]);
    let path_2 = generate_path(&paths[1]);

    let path_1_enumerated = path_1.iter().enumerate();
    let path_2_enumerated = path_2.iter().enumerate();

    let mut sum_of_steps: Vec<usize> = Vec::new();
    for (path_1_step, path_1_pos) in path_1_enumerated {
        let path_2_enumerated = path_2_enumerated.clone();
        for (path_2_step, path_2_pos) in path_2_enumerated {
            if path_1_pos == path_2_pos {
                sum_of_steps.push(path_1_step + path_2_step);
            }
        }
    }

    *sum_of_steps.iter().min().unwrap()
}

fn generate_path(instructions: &Vec<String>) -> Vec<Pos> {
    let mut path: Vec<Pos> = Vec::new();
    let mut x = 0;
    let mut y = 0;

    for instruction in instructions {
        let distance: isize = instruction
            .chars()
            .skip(1)
            .collect::<String>()
            .parse()
            .unwrap();

        let direction = instruction.as_bytes()[0] as char;
        for _i in 0..distance {
            match direction {
                'R' => x += 1,
                'U' => y += 1,
                'L' => x -= 1,
                'D' => y -= 1,
                _ => panic!("Unexpected direction"),
            }

            path.push(Pos { x, y });
        }
    }

    path
}

fn calculate_manhatten_distance(pos: &Pos) -> isize {
    pos.x.abs() + pos.y.abs()
}

#[cfg(test)]
mod day_03_tests {
    // use super::*;

    // #[test]
    // fn part_1_gives_correct_answer() {
    //     assert_eq!(part_1(), 529);
    // }

    // #[test]
    // fn part_2_gives_correct_answer() {
    //     assert_eq!(part_2(), 529);
    // }
}
