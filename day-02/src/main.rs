use utils;

fn main() {
    println!("{}", part_1());
}

fn part_1() -> usize {
    let mut intcode: Vec<usize> = utils::read_string("./input.txt")
        .split(',')
        .map(|a| a.parse().unwrap())
        .collect();

    intcode[1] = 12;
    intcode[2] = 2;

    let mut pc = 0;
    loop {
        match intcode[pc] {
            1 => {
                let output_pos = intcode[pc + 3];
                intcode[output_pos] = intcode[intcode[pc + 1]] + intcode[intcode[pc + 2]]
            }
            2 => {
                let output_pos = intcode[pc + 3];
                intcode[output_pos] = intcode[intcode[pc + 1]] * intcode[intcode[pc + 2]]
            }
            99 => {
                break;
            }
            _ => {}
        }

        pc += 4;
    }

    intcode[0]
}

#[cfg(test)]
mod day_01_tests {
    use super::*;

    #[test]
    fn part_1_gives_correct_answer() {
        assert_eq!(part_1(), 5866663);
    }
}
