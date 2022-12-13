use crate::common::{compare_parts, parse_packet, Part};

pub fn solve(input: &str) -> usize {
    let mut packets: Vec<Part> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(parse_packet)
        .collect();

    let divider1 = parse_packet("[[2]]");
    let divider2 = parse_packet("[[6]]");
    packets.push(divider1.clone());
    packets.push(divider2.clone());

    packets.sort_by(compare_parts);

    let divider1 = packets.iter().position(|p| *p == divider1).unwrap() + 1;
    let divider2 = packets.iter().position(|p| *p == divider2).unwrap() + 1;
    divider1 * divider2
}
