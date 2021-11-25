use utils;

fn main() {
    println!("{}", part_1());
}

fn part_1() -> usize {
    utils::read_lines("./input.txt")
        .map(|a| calculate_required_fuel(a.unwrap().as_str().parse().unwrap()))
        .sum::<usize>()
}

fn calculate_required_fuel(mass: usize) -> usize {
    mass / 3 - 2
}

#[cfg(test)]
mod day_01_tests {
    use super::*;

    #[test]
    fn part_1_gives_correct_answer() {
        assert_eq!(part_1(), 3216744);
    }
}
