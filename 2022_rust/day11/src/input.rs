use std::collections::VecDeque;

#[derive(PartialEq, Debug, Clone)]
pub struct Monkey {
    pub items: VecDeque<usize>,
    pub divisor: usize, // extracted from throw_to, throw_to could be simplified
    pub op: fn(usize) -> usize,
    pub throw_to: fn(usize) -> usize,
}

pub fn get_monkeys() -> Vec<Monkey> {
    return vec![
        Monkey {
            //0
            items: VecDeque::from([89, 84, 88, 78, 70]),
            op: |x| x * 5,
            divisor: 7,
            throw_to: |x| if x % 7 == 0 { 6 } else { 7 },
        },
        Monkey {
            //1
            items: VecDeque::from([76, 62, 61, 54, 69, 60, 85]),
            op: |x| x + 1,
            divisor: 17,
            throw_to: |x| if x % 17 == 0 { 0 } else { 6 },
        },
        Monkey {
            //2
            items: VecDeque::from([83, 89, 53]),
            op: |x| x + 8,
            divisor: 11,
            throw_to: |x| if x % 11 == 0 { 5 } else { 3 },
        },
        Monkey {
            //3
            items: VecDeque::from([95, 94, 85, 57]),
            op: |x| x + 4,
            divisor: 13,
            throw_to: |x| if x % 13 == 0 { 0 } else { 1 },
        },
        Monkey {
            //4
            items: VecDeque::from([82, 98]),
            op: |x| x + 7,
            divisor: 19,
            throw_to: |x| if x % 19 == 0 { 5 } else { 2 },
        },
        Monkey {
            //5
            items: VecDeque::from([69]),
            op: |x| x + 2,
            divisor: 2,
            throw_to: |x| if x % 2 == 0 { 1 } else { 3 },
        },
        Monkey {
            //6
            items: VecDeque::from([82, 70, 58, 87, 59, 99, 92, 65]),
            op: |x| x * 11,
            divisor: 5,
            throw_to: |x| if x % 5 == 0 { 7 } else { 4 },
        },
        Monkey {
            //7
            items: VecDeque::from([91, 53, 96, 98, 68, 82]),
            op: |x| x * x,
            divisor: 3,
            throw_to: |x| if x % 3 == 0 { 4 } else { 2 },
        },
    ];
}
