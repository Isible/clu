use std::collections::VecDeque;

use strsim::levenshtein;

pub fn find_most_similar_string(string_a: &str, string_list: Vec<String>) -> Option<String> {
    let mut min_distance = usize::MAX;
    let mut most_similar_string = None;

    for string_b in string_list {
        let distance = levenshtein(&string_a, &string_b);

        if distance < min_distance {
            min_distance = distance;
            most_similar_string = Some(string_b);
        }
    }

    most_similar_string
}

// converts a one digit long, unsigned integer into a character
pub fn uint_to_char(from: u8) -> char {
    match from {
        0 => '0',
        1 => '1',
        2 => '2',
        3 => '3',
        4 => '4',
        5 => '5',
        6 => '6',
        7 => '7',
        8 => '8',
        9 => '9',
        _ => todo!(),
    }
}

/// converts numbers from 0-18 (the largest result that can be achieved
/// in an addition of two single digit numbers) to a vector of characters 
pub fn uint_to_short_vec<'a>(from: u8) -> Vec<char> {
    match from {
        0 => vec!['0'],
        1 => vec!['1'],
        2 => vec!['2'],
        3 => vec!['3'],
        4 => vec!['4'],
        5 => vec!['5'],
        6 => vec!['6'],
        7 => vec!['7'],
        8 => vec!['8'],
        9 => vec!['9'],
        10 => vec!['1', '0'],
        11 => vec!['1', '1'],
        12 => vec!['1', '2'],
        13 => vec!['1', '3'],
        14 => vec!['1', '4'],
        15 => vec!['1', '5'],
        16 => vec!['1', '6'],
        17 => vec!['1', '7'],
        18 => vec!['1', '8'],
        _ => todo!(),
    }
}

pub fn adjust_int_length((first_int, second_int): (String, String)) -> (String, String) {
    return match get_longer_string(&first_int, &second_int) {
        0 => {
            let length = first_int.len() - second_int.len();
            let mut string: VecDeque<char> = second_int.chars().collect();

            for _ in 0..length {
                string.push_front('0');
            }

            (first_int, string.iter().collect())
        },
        1 => {
            let length = second_int.len() - first_int.len();
            let mut string: VecDeque<char> = first_int.chars().collect();

            for _ in 0..length {
                string.push_front('0');
            }

            (string.iter().collect(), second_int)
        },
        _ => panic!("Unreachable")
    }
}

pub fn get_longer_string(string1: &String, string2: &String) -> usize {
    if string1.len() > string2.len() {
        return 0;
    }
    return 1;
}
