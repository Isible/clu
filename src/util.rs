use std::string::ParseError;

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

pub fn uint_to_char(from: u32) -> char {
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
        _ => todo!()
    }
}