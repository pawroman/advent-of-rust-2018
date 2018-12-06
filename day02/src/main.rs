use std::env;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use common::{get_input, Error};


fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let input = get_input(&args);

    if input.is_err() {
        eprintln!("Error: {}. Aborting.", input.unwrap_err());
        std::process::exit(1);
    }

    let strings: Vec<String> = input.unwrap();

    // part 1

    println!("Checksum: {}", checksum(&strings));

    // part 2

    match common_string_parts(&strings, 1) {
        ref common if common.len() == 1 => println!("Common part: {}", common[0].common),
        ref common if common.len() == 0 => {
            println!("No common parts with one difference!");
            std::process::exit(2);
        }
        ref common => {
            println!("Unexpected number of answers: {} (expected 1)!", common.len());
            std::process::exit(2);
        }
    }

    Ok(())
}


fn checksum<T>(values: &[T]) -> usize
    where T: AsRef<str>
{
    if values.is_empty() {
        return 0;
    }

    // checksum == (number of values than contain
    //              doubly repeated items) * (triple repeats)
    let mut double_repeats = 0;
    let mut triple_repeats = 0;

    for value in values {
        let counts = count_items(value.as_ref().chars());
        let unique_counts: HashSet<_> = counts.values().collect();

        if unique_counts.contains(&2) {
            double_repeats += 1
        }

        if unique_counts.contains(&3) {
            triple_repeats += 1
        }
    }

    double_repeats * triple_repeats
}


fn count_items<I>(values: impl IntoIterator<Item=I>) -> HashMap<I, usize>
    where I: Eq + Hash,
{
    let mut counts = HashMap::new();

    for item in values {
        counts
            .entry(item)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    counts
}


#[derive(Debug, PartialEq, Eq)]
struct CommonString<'a> {
    left: &'a str,
    right: &'a str,
    common: String,
}


fn common_string_parts<T>(strings: &[T], differences: usize) -> Vec<CommonString>
    where T: AsRef<str>
{
    let mut result = vec![];

    // 2-combinations of all strings
    for i in 0..strings.len() {
        for j in i+1..strings.len() {
            let left = strings[i].as_ref();
            let right = strings[j].as_ref();

            if let Some(common) = common_string_part(left, right, differences) {
                result.push(CommonString { left, right, common })
            }
        }
    }

    result
}


fn common_string_part(left: &str, right: &str, differences: usize) -> Option<String> {
    if left.len() != right.len() {
        // DANGER: not UTF friendly
        return None;
    }

    let mut common_chars = vec![];

    let zip_chars = left.chars()
        .zip(right.chars());

    for (left_char, right_char) in zip_chars {
        if left_char == right_char {
            common_chars.push(left_char);
        }
    }

    if left.len() - common_chars.len() == differences {
        Some(common_chars.into_iter().collect())
    } else {
        None
    }
}


#[cfg(test)]
mod tests {
    use super::{checksum, count_items, common_string_parts, CommonString};

    use std::collections::HashMap;

    #[test]
    fn test_checksum() {
        assert_eq!(
            checksum(&[
                "abcdef", "bababc", "abbcde", "abcccd",
                "aabcdd", "abcdee", "ababab",
            ]),
            12
        );
    }

    #[test]
    fn test_count_items() {
        let mut expected_counts = HashMap::new();

        expected_counts.insert(1, 2 as usize);
        expected_counts.insert(2, 1);
        expected_counts.insert(3, 1);
        expected_counts.insert(5, 4);

        let stuff = count_items([1, 1, 2, 3, 5, 5, 5, 5].iter());

        // poor man's hashmap equality comparison
        assert_eq!(expected_counts.len(), stuff.len());

        for (k, v) in expected_counts {
            assert_eq!(stuff.get(&k).unwrap(), &v);
        }
    }

    #[test]
    fn test_common_string_parts() {
        let strings = [
            "abcde", "fghij", "klmno", "pqrst",
            "fguij", "axcye", "wvxyz"
        ];

        assert_eq!(
            common_string_parts(&strings, 1),
            vec![
                CommonString { common: "fgij".into(), left: "fghij", right: "fguij"}
            ]
        );

        assert_eq!(
            common_string_parts(&strings, 2),
            vec![
                CommonString { common: "ace".into(), left: "abcde", right: "axcye"}
            ]
        );
    }

    #[test]
    fn test_common_string_parts_different_lengths() {
        let strings = ["abc", "abdef"];

        assert_eq!(
            common_string_parts(&strings, 1),
            vec![]
        );

        assert_eq!(
            common_string_parts(&strings, 2),
            vec![]
        );

        assert_eq!(
            common_string_parts(&strings, 3),
            vec![]
        );
    }
}
