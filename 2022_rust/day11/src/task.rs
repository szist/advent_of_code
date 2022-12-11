use crate::input::{get_monkeys, Monkey};

#[derive(PartialEq)]
pub enum InspectMode {
    DIVIDE,
    NODIVIDE(usize),
}

fn inspect(monkey: &Monkey, item: usize, mode: &InspectMode) -> (usize, usize) {
    let worry_level = (monkey.op)(item);
    let worry_level = match mode {
        InspectMode::DIVIDE => worry_level / 3,
        InspectMode::NODIVIDE(x) => worry_level % x,
    };
    let throw_to = (monkey.throw_to)(worry_level);
    (worry_level, throw_to)
}

pub fn solve(mode: InspectMode) -> usize {
    let mut monkeys = get_monkeys();
    let count = monkeys.len();
    let mut inspections = vec![0; count];
    let rounds = match mode {
        InspectMode::DIVIDE => 20,
        InspectMode::NODIVIDE(_) => 10_000,
    };
    for _round in 0..rounds {
        // 20 rounds
        for monkey in 0..count {
            while let Some(item) = monkeys[monkey].items.pop_back() {
                inspections[monkey] += 1;
                let (worry_level, throw_to) = inspect(&monkeys[monkey], item, &mode);
                monkeys[throw_to].items.push_front(worry_level);
            }
        }
    }
    println!("Inspections: {:?}", inspections);
    inspections.sort_by(|a, b| b.cmp(a));
    inspections[0] * inspections[1]
}
