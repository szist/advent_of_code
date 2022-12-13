use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Part {
    pub is_list: bool,
    pub val: usize,
    pub list: Vec<Part>,
}

fn find_ending(arr: &str) -> usize {
    let mut count = 0;
    for (index, char) in arr.chars().enumerate() {
        match char {
            ']' => count -= 1,
            '[' => count += 1,
            _ => (),
        }
        if count == 0 {
            return index;
        }
    }
    0
}

fn parse_field(fields: &str) -> (Part, &str) {
    let first_char = fields.chars().next().unwrap();
    match first_char {
        '[' => {
            let end = find_ending(fields);
            (parse_arr(&fields[..=end]), &fields[end + 1..])
        }
        x if x.is_numeric() => {
            let (num, _) = fields.split_once(|x: char| !x.is_numeric()).unwrap();
            let val: usize = num.parse().unwrap();
            (
                Part {
                    is_list: false,
                    val,
                    list: vec![],
                },
                &fields[num.len()..],
            )
        }
        x => unreachable!("Panic! {x} {fields}"),
    }
}

// expects [XXX]
fn parse_arr(arr: &str) -> Part {
    let mut fields: Vec<Part> = vec![];
    let mut start = 1;
    while start != arr.len() - 1 {
        let (field, rest) = parse_field(&arr[start..]);

        fields.push(field);
        if !rest.starts_with(',') {
            break;
        } else {
            start = arr.len() - rest.len() + 1
        }
    }
    Part {
        is_list: true,
        val: 0,
        list: fields,
    }
}

pub fn parse_packet(packet: &str) -> Part {
    parse_arr(packet)
}

// This would be nicer with a trait probably
pub fn compare_parts(a: &Part, b: &Part) -> Ordering {
    match (a.is_list, b.is_list) {
        (false, false) => a.val.cmp(&b.val),
        (false, true) => compare_parts(
            &Part {
                is_list: true,
                val: 0,
                list: vec![a.clone()],
            },
            b,
        ),
        (true, false) => compare_parts(
            a,
            &Part {
                is_list: true,
                val: 0,
                list: vec![b.clone()],
            },
        ),
        (true, true) => {
            let mut a_fields = a.list.iter();
            let mut b_fields = b.list.iter();
            let mut order = Ordering::Equal;
            while order == Ordering::Equal {
                order = match (a_fields.next(), b_fields.next()) {
                    (Some(p), Some(k)) => compare_parts(p, k),
                    (None, Some(_)) => Ordering::Less,
                    (Some(_), None) => Ordering::Greater,
                    (None, None) => break,
                }
            }
            order
        }
    }
}

pub fn compare_packets(a: &str, b: &str) -> Ordering {
    compare_parts(&parse_packet(a), &parse_packet(b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple() {
        let packet = parse_packet("[[4,4],4,4]");
        println!("Packet {:#?}", &packet);
        assert_eq!(packet.is_list, true);
    }

    #[test]
    fn check_comparison() {
        assert_eq!(
            compare_parts(&parse_packet("[]"), &parse_packet("[[]]")),
            Ordering::Less
        )
    }

    #[test]
    fn check_flat() {
        assert_eq!(
            compare_packets("[1,1,3,1,1]", "[1,1,5,1,1]"),
            Ordering::Less
        )
    }

    #[test]
    fn check_convert() {
        assert_eq!(compare_packets("[[1],[2,3,4]]", "[[1],4]"), Ordering::Less)
    }

    #[test]
    fn check_convert_rev() {
        assert_eq!(
            compare_packets("[[1],4]", "[[1],[2,3,4]]"),
            Ordering::Greater
        )
    }

    #[test]
    fn check_single() {
        assert_eq!(compare_packets("[9]", "[[8,7,6]]"), Ordering::Greater)
    }

    #[test]
    fn check_shorter() {
        assert_eq!(
            compare_packets("[[4,4],4,4]", "[[4,4],4,4,4]"),
            Ordering::Less
        );
    }

    #[test]
    fn check_longer() {
        assert_eq!(
            compare_packets("[[4,4],4,4,4]", "[[4,4],4,4]"),
            Ordering::Greater
        );
    }

    #[test]
    fn check_empty() {
        assert_eq!(compare_packets("[]", "[3]"), Ordering::Less);
    }

    #[test]
    fn check_empty_long() {
        assert_eq!(compare_packets("[1,[0]]", "[[],[2,3]]"), Ordering::Greater);
    }

    #[test]
    fn check_deep_empty() {
        assert_eq!(compare_packets("[1,[],1]", "[1,[4]]"), Ordering::Less);
    }

    #[test]
    fn check_wrong() {
        assert_eq!(
            compare_packets(
                "[[[[10,6,6,10],[],[],8,5],[[8,10],10],10],[6,[5,8,[],[]]],[[0],1]]",
                "[[[6],[2,[8,2],8,5,[6,9,4]],[[],10,[9,1,9,9]]],[8,[],10,[[0]],[[]]],[],[4]]"
            ),
            Ordering::Greater
        )
    }
}
