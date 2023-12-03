pub fn read_puzzle_input(fd: &mut dyn std::io::Read) -> std::io::Result<Vec<String>> {
    let mut data = Default::default();
    fd.read_to_string(&mut data)?;
    Ok(data.lines().map(|x| x.to_string()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_puzzle_input_empty_file() {
        // Setup
        let mut input = std::io::Cursor::new(Vec::<u8>::new());
        // Test
        let lines = read_puzzle_input(&mut input).expect("Failed to read puzzle input");
        // Assert
        assert!(lines.is_empty());
    }

    #[test]
    fn test_read_puzzle_input_1line_lf() {
        // Setup
        let mut input = std::io::Cursor::new("abcd\n");
        let expected = vec!["abcd"];
        // Test
        let lines = read_puzzle_input(&mut input).expect("Failed to read puzzle input");
        // Assert
        assert_eq!(lines, expected);
    }

    #[test]
    fn test_read_puzzle_input_1line_crlf() {
        // Setup
        let mut input = std::io::Cursor::new("abcd\r\n");
        let expected = vec!["abcd"];
        // Test
        let lines = read_puzzle_input(&mut input).expect("Failed to read puzzle input");
        // Assert
        assert_eq!(lines, expected);
    }

    #[test]
    fn test_read_puzzle_input_2line_mixed() {
        // Setup
        let mut input = std::io::Cursor::new("abcd\n1234\r\n");
        let expected = vec!["abcd", "1234"];
        // Test
        let lines = read_puzzle_input(&mut input).expect("Failed to read puzzle input");
        // Assert
        assert_eq!(lines, expected);
    }
}
