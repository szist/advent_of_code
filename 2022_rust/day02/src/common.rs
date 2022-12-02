
pub fn parse_line(line: &str) -> Result<(u32, u32), &'static str> {
    let trimmed = line.trim();
    if trimmed.len() != 3 {
        return Err("Unexpected line");
    }
    let mut chars = trimmed.chars();
    let opponent = chars.next().unwrap();
    // empty space
    chars.next();
    let mine = chars.next().unwrap();
    return Ok((opponent as u32, mine as u32));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_line_test() {
        assert_eq!(parse_line("A X"), Ok(('A' as u32, 'X' as u32)));
    }
}