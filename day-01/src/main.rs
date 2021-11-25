use utils;

fn main() {
    println!("{}", part_1());
    println!("{}", part_2());
}

fn part_1() -> isize {
    utils::read_lines("./input.txt")
        .map(|mass_input| calculate_required_fuel(mass_input.unwrap().as_str().parse().unwrap()))
        .sum()
}

fn part_2() -> isize {
    utils::read_lines("./input.txt")
        .map(|mass_input| {
            let mut fuel: isize = 0;
            let mass: isize = mass_input.unwrap().as_str().parse().unwrap();

            let mut extra_fuel: isize = mass as isize;
            loop {
                extra_fuel = calculate_required_fuel(extra_fuel);

                if extra_fuel <= 0 {
                    break;
                }

                fuel += extra_fuel;
            }

            fuel
        })
        .sum()
}

fn calculate_required_fuel(mass: isize) -> isize {
    mass / 3 - 2
}

#[cfg(test)]
mod day_01_tests {
    use super::*;

    #[test]
    fn part_1_gives_correct_answer() {
        assert_eq!(part_1(), 3216744);
    }

    #[test]
    fn part_2_gives_correct_answer() {
        assert_eq!(part_2(), 4822249);
    }
}
