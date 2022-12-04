use super::common::parse_ranges;

pub fn solve(input: &String) -> u32 {
    let mut overlap = 0;
    for line in input.trim_end().split('\n') {
        let (section_a, section_b) = parse_ranges(line);
        let range_a = section_a.0..=section_a.1;
        let range_b = section_b.0..=section_b.1;
        if (range_a.contains(range_b.start()) && range_a.contains(range_b.end()))
            || (range_b.contains(range_a.start()) && range_b.contains(range_a.end()))
        {
            overlap += 1;
        }
    }
    overlap
}
