use advent_of_code_2023::read_puzzle_input;

fn line_to_digits(line: &str) -> Vec<u8> {
    let mut digits: Vec<u8> = Default::default();
    let mut it = line.as_bytes().iter().enumerate();
    while let Some((idx, char)) = it.next() {
        match char {
            _ if char.is_ascii_digit() => {
                digits.push(*char);
            }
            b'o' => {
                if line[idx..].starts_with("one") {
                    it.next(); // n
                    it.next(); // e
                    digits.push(b'1');
                }
            }
            b't' => {
                let substr = &line[idx..];
                if substr.starts_with("two") {
                    it.next(); // w
                    it.next(); // o
                    digits.push(b'2');
                } else if substr.starts_with("three") {
                    it.next(); // h
                    it.next(); // r
                    it.next(); // e
                    it.next(); // e
                    digits.push(b'3');
                }
            }
            b'f' => {
                let substr = &line[idx..];
                if substr.starts_with("four") {
                    it.next(); // o
                    it.next(); // u
                    it.next(); // r
                    digits.push(b'4');
                } else if substr.starts_with("five") {
                    it.next(); // i
                    it.next(); // v
                    it.next(); // e
                    digits.push(b'5');
                }
            }
            b's' => {
                let substr = &line[idx..];
                if substr.starts_with("six") {
                    it.next(); // i
                    it.next(); // x
                    digits.push(b'6');
                } else if substr.starts_with("seven") {
                    it.next(); // e
                    it.next(); // v
                    it.next(); // e
                    it.next(); // n
                    digits.push(b'7');
                }
            }
            b'e' => {
                if line[idx..].starts_with("eight") {
                    it.next(); // i
                    it.next(); // g
                    it.next(); // h
                    it.next(); // t
                    digits.push(b'8');
                }
            }
            b'n' => {
                if line[idx..].starts_with("nine") {
                    it.next(); // i
                    it.next(); // n
                    it.next(); // e
                    digits.push(b'9');
                }
            }
            _ => {
                // Don't care
                continue;
            }
        };
    }
    digits
}

fn process_line(line: &str) -> Option<(i32, i32)> {
    let digits = line_to_digits(line);
    // Convert char value by subtracting the ascii value of '0'
    let n1 = digits.first()? - b'0';
    let n2 = digits.last()? - b'0';
    Some((n1 as i32, n2 as i32))
}

fn translate_instructions(lines: Vec<String>) -> i32 {
    lines
        .into_iter()
        .filter_map(|line| process_line(&line))
        .map(|(d1, d2)| (d1 * 10) + d2)
        .sum()
}

fn main() {
    let lines = read_puzzle_input(&mut std::io::stdin()).expect("Unable to read puzzle input");
    let result = translate_instructions(lines);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_line_empty() {
        // Setup
        let input = "";
        // Test
        let result = process_line(input);
        // Assert
        assert!(result.is_none());
    }

    #[test]
    fn process_line_1digit() {
        // Setup
        let input = "1";
        let expected = (1, 1);
        // Test
        let result = process_line(input).expect("Line processing failed");
        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn process_line_complex() {
        // Setup
        let input = "a1bc2dq3z";
        let expected = (1i32, 3i32);
        // Test
        let result = process_line(input).expect("Line processing failed");
        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn process_line_words() {
        // Setup
        let input = "six1b2dseventeen";
        let expected = (6, 7);
        // Test
        let result = process_line(input).expect("Line processing failed");
        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn process_line_words_share_character() {
        // Setup
        let input = "twone3";
        let expected = (2, 3);
        // Test
        let result = process_line(input).expect("Line processing failed");
        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn translate_instructions_example() {
        // Setup
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet\n";
        let lines = read_puzzle_input(&mut std::io::Cursor::new(input))
            .expect("Unable to read puzzle input");
        let expected = 142;
        // Test
        let result = translate_instructions(lines);
        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn translate_instructions_example2() {
        // Setup
        let input = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen\n";
        let lines = read_puzzle_input(&mut std::io::Cursor::new(input))
            .expect("Unable to read puzzle input");
        let expected = 281;
        // Test
        let result = translate_instructions(lines);
        // Assert
        assert_eq!(result, expected);
    }
}
