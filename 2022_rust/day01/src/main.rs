use std::fs::File;
use std::io::{self, BufRead};
use std::env::{args};
use std::path::Path;

/**
 * I feel like a noob trying to write C in rust. I love it XD
 */
fn main() {
    let args: Vec<String> = args().collect();

    let input_file = &args[1];

    println!("Reading file: {}", input_file);
    let mut top_calories = vec![0, 0, 0];
    if let Ok(lines) = read_lines(input_file) {
        let mut current_elf_calories_sum: i32 = 0;
        for line in lines {
            if let Ok(calory_line) = line {
                if calory_line.len() > 0 {
                    let calorie: i32 = calory_line.parse().unwrap();
                    current_elf_calories_sum += calorie;
                } else {
                    if top_calories[0] < current_elf_calories_sum {
                        top_calories[0] = current_elf_calories_sum;
                        top_calories.sort();
                    }
                    current_elf_calories_sum = 0;
                }
            }
        }
        // last elf, this is copy paste and should be correctly extracted, but meh
        if top_calories[0] < current_elf_calories_sum {
            top_calories[0] = current_elf_calories_sum;
            top_calories.sort()
        }
    }

    println!("Maximum calories: {}", top_calories[2]);
    println!("Top 3 sum {}", top_calories[2] + top_calories[1] + top_calories[0])
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
