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
}

#[derive(Clone, Copy, PartialEq, Debug, Eq)]
struct Pos {
    x: isize,
    y: isize,
}

// #[derive(PartialEq)]
// enum Orientation {
//     Horizontal,
//     Vertical,
// }

fn part_1() -> isize {
    let paths: Vec<Vec<String>> = utils::read_string("./input.txt")
        .split("\n")
        .map(|a| a.split(",").map(|a| a.to_string()).collect())
        .collect();

    let path_1 = generate_path(&paths[0]);
    let path_2 = generate_path(&paths[1]);

    let intersections: Vec<&Pos> = path_1.iter().filter(|&pos| path_2.contains(pos)).collect();

    // for i in 0..(path_1.len() - 1) {
    //     let line_1 = (path_1[i], path_1[i + 1]);
    //     let line_1_orientation = get_orientation(line_1);

    //     for j in 0..(path_2.len() - 1) {
    //         let line_2 = (path_2[j], path_2[j + 1]);
    //         let line_2_orientation = get_orientation(line_2);

    //         if line_1_orientation == line_2_orientation {
    //             match line_1_orientation {
    //                 Orientation::Vertical => {
    //                     if line_1.0.x == line_2.0.x {
    //                         if line_1.0.y < line_1.1.y {
    //                             if line_2.0.y.is_between(line_1.0.y, line_1.1.y) {
    //                             } else if line_2.1.y.is_between(line_1.0.y, line_1.1.y) {
    //                             }
    //                         } else {
    //                             return ((b.y1 > a.y2 && b.y1 < a.y1)
    //                                 || (b.y2 > a.y2 && b.y2 < a.y1));
    //                         }
    //                     }
    //                 }
    //                 Orientation::Horizontal => if line_1.0.y == line_2.0.y {},
    //             }
    //         }
    //     }
    // }

    intersections
        .iter()
        .map(|&pos| calculate_manhatten_distance(pos))
        .min()
        .unwrap()
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

// fn generate_path(instructions: &Vec<String>) -> Vec<Pos> {
//     let mut path: Vec<Pos> = Vec::new();
//     let mut x = 0;
//     let mut y = 0;
//     path.push(Pos { x, y });
//     for instruction in instructions {
//         let distance: usize = instruction
//             .chars()
//             .skip(1)
//             .collect::<String>()
//             .parse()
//             .unwrap();

//         match instruction.as_bytes()[0] as char {
//             'R' => x += distance,
//             'U' => y += distance,
//             'L' => x -= distance,
//             'D' => y -= distance,
//             _ => panic!("Unexpected "),
//         }

//         path.push(Pos { x, y });
//     }

//     path
// }

// fn get_orientation(line: (Pos, Pos)) -> Orientation {
//     if line.0.x == line.1.x {
//         Orientation::Vertical
//     } else {
//         Orientation::Horizontal
//     }
// }

fn calculate_manhatten_distance(pos: &Pos) -> isize {
    pos.x.abs() + pos.y.abs()
}

trait Bound {
    fn is_between(self: Self, start: usize, end: usize) -> bool;
}

impl Bound for usize {
    fn is_between(self: Self, start: usize, end: usize) -> bool {
        self > start && self < end
    }
}

#[cfg(test)]
mod day_03_tests {
    use super::*;

    #[test]
    fn part_1_gives_correct_answer() {
        assert_eq!(part_1(), 529);
    }
}
