fn main() {
    println!("{}", part_1());
    println!("{}", part_2());
}

fn part_1() -> usize {
    let range = 245318..765747;

    let mut valid_passwords = 0;

    for num in range {
        let num_string = num.to_string();
        let digits = num_string.as_bytes();

        if has_same_adjacent_digits(digits) && digits_never_decrease(digits) {
            valid_passwords += 1;
        }
    }

    valid_passwords
}

fn part_2() -> usize {
    let range = 245318..765747;

    let mut valid_passwords = 0;

    for num in range {
        let num_string = num.to_string();
        let digits = num_string.as_bytes();

        if has_separate_adjacent_digits(digits) && digits_never_decrease(digits) {
            valid_passwords += 1;
        }
    }

    valid_passwords
}

fn has_same_adjacent_digits(digits: &[u8]) -> bool {
    for n in 1..digits.len() {
        if digits[n - 1] == digits[n] {
            return true;
        }
    }

    false
}

fn digits_never_decrease(digits: &[u8]) -> bool {
    for n in 1..digits.len() {
        if digits[n - 1] > digits[n] {
            return false;
        }
    }

    true
}

fn has_separate_adjacent_digits(digits: &[u8]) -> bool {
    let mut has_same_adjacent_digits = false;
    let mut lock = false;
    // let mut last_same_digit_pos = 0usize;
    // let mut last_same_digit = 0u8;
    for n in 1..digits.len() {
        if digits[n - 1] == digits[n] {
            if has_same_adjacent_digits {
                if !lock {
                    has_same_adjacent_digits = false;
                    lock = true;
                }
            } else {
                if !lock {
                    has_same_adjacent_digits = true;
                }
            }

            // if has_same_adjacent_digits {
            //     if last_same_digit_pos == n - 1 && last_same_digit == digits[n] {
            //         has_same_adjacent_digits = false;
            //         last_same_digit_pos = n;
            //     }
            // } else {
            //     has_same_adjacent_digits = true;
            // }
        } else {
            lock = false;
        }
    }

    has_same_adjacent_digits
}

#[cfg(test)]
mod day_04_tests {
    use super::*;

    #[test]
    fn part_1_gives_correct_answer() {
        assert_eq!(part_1(), 1079)
    }

    #[test]
    fn has_separate_adjacent_digits_example_1() {
        let num = 112233;
        let num_string = num.to_string();
        let digits = num_string.as_bytes();

        assert_eq!(has_separate_adjacent_digits(digits), true)
    }

    #[test]
    fn has_separate_adjacent_digits_example_2() {
        let num = 123444;
        let num_string = num.to_string();
        let digits = num_string.as_bytes();

        assert_eq!(has_separate_adjacent_digits(digits), false)
    }

    #[test]
    fn has_separate_adjacent_digits_example_3() {
        let num = 111122;
        let num_string = num.to_string();
        let digits = num_string.as_bytes();

        assert_eq!(has_separate_adjacent_digits(digits), true)
    }
}
