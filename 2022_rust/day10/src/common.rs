pub enum Instruction {
    Noop,
    Addx(isize),
}

pub fn parse_instructions(input: &str) -> Vec<Instruction> {
    input
        .trim_end()
        .lines()
        .map(|line| {
            let parts = line.split_once(' ');
            match parts {
                None => Instruction::Noop,
                Some((_, x)) => Instruction::Addx(x.parse().unwrap()),
            }
        })
        .collect()
}

pub fn instruction_cycles(instruction: &Instruction) -> usize {
    match instruction {
        Instruction::Noop => 1,
        Instruction::Addx(_) => 2,
    }
}

pub fn program_length(instructions: &[Instruction]) -> usize {
    instructions.iter().map(instruction_cycles).sum()
}

pub fn register_changes(instructions: &[Instruction]) -> Vec<isize> {
    let mut changes = vec![0; program_length(instructions) + 1];
    changes[0] = 1;

    let mut current_cycle = 0;
    for inst in instructions {
        current_cycle += instruction_cycles(inst);
        match inst {
            Instruction::Noop => (),
            Instruction::Addx(v) => changes[current_cycle] = *v,
        }
    }
    changes
}

pub fn register_states(instructions: &[Instruction]) -> Vec<isize> {
    register_changes(instructions)
        .iter()
        .scan(0, |sum, i| {
            *sum += i;
            Some(*sum)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_changes() {
        let changes = register_changes(&vec![
            Instruction::Noop,
            Instruction::Addx(3),
            Instruction::Addx(-5),
        ]);
        assert_eq!(3 as isize, changes[3]);
        assert_eq!(-5 as isize, changes[5]);
    }

    #[test]
    fn check_states() {
        let changes = register_states(&vec![
            Instruction::Noop,
            Instruction::Addx(3),
            Instruction::Addx(-5),
        ]);
        assert_eq!(1 as isize, changes[2]);
        assert_eq!(4 as isize, changes[3]);
        assert_eq!(4 as isize, changes[4]);
        assert_eq!(-1 as isize, changes[5]);
    }
}
