use crate::common::{register_states, Instruction};

pub fn solve(instructions: &[Instruction]) {
    let states = register_states(instructions);
    const DISPLAY_WIDTH: usize = 40;
    for row_cycle_start in (1..=240).step_by(DISPLAY_WIDTH) {
        print!("#");
        for pixel in 0..DISPLAY_WIDTH {
            let sprite_pos = states[row_cycle_start + pixel - 1] as usize;
            if pixel >= sprite_pos - 1 && pixel <= sprite_pos + 1 {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!("#");
    }
}
