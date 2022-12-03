use std::collections::HashSet;

pub fn parse_rucksack(rucksack: &str) -> HashSet<char> {
    rucksack.trim().chars().collect()
}

pub fn parse_compartments(rucksack: &str) -> (HashSet<char>, HashSet<char>) {
    let (a, b) = rucksack.trim().split_at(rucksack.len()/2);
    let comp_a: HashSet<char> = a.chars().collect();
    let comp_b: HashSet<char> = b.chars().collect();
    (comp_a, comp_b)
}

pub fn compute_priority(item: char) -> u32 {
    let item_code = item as u32;
    match item {
        'a'..='z' => item_code - ('a' as u32) + 1,
        'A'..='Z' => item_code - ('A' as u32) + 27,
        _ => panic!("Bad item")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_parsing() {
        assert_eq!(parse_compartments("aabb"), (HashSet::from(['a']), HashSet::from(['b'])));
    }

    #[test]
    fn test_priority() {
        assert_eq!(compute_priority('a'), 1);
        assert_eq!(compute_priority('z'), 26);
        assert_eq!(compute_priority('A'), 27);
        assert_eq!(compute_priority('Y'), 51);
    }
}