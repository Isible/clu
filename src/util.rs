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