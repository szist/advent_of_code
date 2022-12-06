use std::collections::{HashSet, VecDeque};

fn has_duplicate(window: &VecDeque<char>) -> bool {
    let set: HashSet<&char> = HashSet::from_iter(window.iter());
    set.len() != window.len()
}

pub fn unique_window_start(input: &str, window_size: usize) -> usize {
    let mut window: VecDeque<char> = VecDeque::from_iter(input.chars().take(window_size));

    for (i, v) in input.trim().chars().enumerate() {
        window.pop_front();
        window.push_back(v);

        // duplicate detection could be done by not recreating hashsets every time,
        // but could use a hashmap instead with counts and verify key count & remove keys with 0 val
        // or just track the duplicate state and check in linear time if the new entry
        // would create a duplicate in the window
        // or run 2 iterators in parallel to avoid also storing the window explicitely
        if !has_duplicate(&window) {
            return i + 1;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_solve1() {
        assert_eq!(unique_window_start("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
    }

    #[test]
    fn check_solve2() {
        assert_eq!(
            unique_window_start("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4),
            10
        );
    }

    #[test]
    fn check_solve_long1() {
        assert_eq!(
            unique_window_start("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14),
            19
        );
    }
}
