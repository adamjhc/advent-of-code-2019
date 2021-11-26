use utils;

fn main() {
    println!("{}", part_1());
    println!("{}", part_2());
}

fn part_1() -> usize {
    let mut memory = read_memory();

    memory[1] = 12;
    memory[2] = 2;

    memory = run_intcode(memory);

    memory[0]
}

fn part_2() -> usize {
    let memory_base = read_memory();

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut memory = memory_base.clone();

            memory[1] = noun;
            memory[2] = verb;

            memory = run_intcode(memory);

            if memory[0] == 19690720 {
                return 100 * noun + verb;
            }
        }
    }

    panic!("No answer found :/")
}

fn read_memory() -> Vec<usize> {
    utils::read_string("./input.txt")
        .split(',')
        .map(|a| a.parse().unwrap())
        .collect()
}

fn run_intcode(mut memory: Vec<usize>) -> Vec<usize> {
    let mut instruction_pointer = 0;
    loop {
        // match opcode
        match memory[instruction_pointer] {
            1 => {
                let output_addr = memory[instruction_pointer + 3];
                memory[output_addr] = memory[memory[instruction_pointer + 1]]
                    + memory[memory[instruction_pointer + 2]]
            }
            2 => {
                let output_addr = memory[instruction_pointer + 3];
                memory[output_addr] = memory[memory[instruction_pointer + 1]]
                    * memory[memory[instruction_pointer + 2]]
            }
            99 => {
                break;
            }
            _ => {}
        }

        instruction_pointer += 4;
    }

    memory
}

#[cfg(test)]
mod day_01_tests {
    use super::*;

    #[test]
    fn part_1_gives_correct_answer() {
        assert_eq!(part_1(), 5866663);
    }

    #[test]
    fn part_2_gives_correct_answer() {
        assert_eq!(part_2(), 4259);
    }
}
