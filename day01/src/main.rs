use std::collections::HashSet;
use std::env;
use std::hash::Hash;
use std::ops::AddAssign;

use common::{get_input, Error};


// maximum number of cycles to allow when looking for repeated sums
const MAX_CYCLES: usize = 1000;


fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let input = get_input(&args);

    if input.is_err() {
        eprintln!("Error: {}. Aborting.", input.unwrap_err());
        std::process::exit(1);
    }

    let frequencies = input.unwrap();

    // part 1

    let freq_sum: i64 = frequencies.iter()
        .sum();

    println!("Sum of frequencies: {}", freq_sum);

    // part 2

    match find_first_cycled_sum_repeat(&frequencies, MAX_CYCLES) {
        Some(repeat) => println!("First repeating frequency: {}", repeat),
        None => {
            println!("No repeats after {} cycles!", MAX_CYCLES);
            std::process::exit(2);
        },
    }

    Ok(())
}


fn find_first_cycled_sum_repeat<T>(values: &[T], max_cycles: usize) -> Option<T>
    where T: AddAssign + Copy + Default + Eq + Hash
{
    if values.is_empty() {
        return None;
    }

    let max_iterations = (max_cycles + 1) * values.len();

    let running_cycled_sum = values.iter()
        .cycle()
        .enumerate()
        // starting with default value for T, produce a running sum
        .scan(T::default(), |sum, (iter_no, val)| {
            // max number of cycles reached, stop iteration
            if iter_no >= max_iterations {
                return None;
            }

            *sum += *val;
            Some(*sum)
        });

    let mut seen = HashSet::new();
    seen.insert(T::default());

    running_cycled_sum
        .skip_while(|sum| {
            // insert will return true on duplicate
            seen.insert(*sum)
        })
        .next()
}


#[cfg(test)]
mod tests {
    use super::find_first_cycled_sum_repeat;

    #[test]
    fn test_find_first_cycled_sum_repeat() {
        assert_eq!(
            find_first_cycled_sum_repeat(&[] as &[i32], 0),
            None
        );

        assert_eq!(
            // doesn't need to cycle, first cycling sum is found for last
            find_first_cycled_sum_repeat(&[1, -1], 0),
            Some(0)
        );

        assert_eq!(
            find_first_cycled_sum_repeat(&[1, -1], 1),
            Some(0)
        );

        assert_eq!(
            find_first_cycled_sum_repeat(&[3, 3, 4, -2, -4], 0),
            None
        );

        assert_eq!(
            find_first_cycled_sum_repeat(&[3, 3, 4, -2, -4], 1),
            Some(10)
        );

        assert_eq!(
            find_first_cycled_sum_repeat(&[7, 7, -2, -7, -4], 2),
            Some(14)
        );
    }
}
