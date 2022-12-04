pub fn parse_ranges(line: &str) -> ((i32, i32), (i32, i32)) {
    let ranges = line
        .split(',')
        .map(|range| {
            let edges = range
                .split('-')
                .map(|num| num.to_string().parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            if edges.len() != 2 {
                panic!("Bad range")
            }
            (edges[0], edges[1])
        })
        .collect::<Vec<_>>();
    if ranges.len() != 2 {
        panic!("Bad input line")
    }
    (ranges[0], ranges[1])
}