use std::collections::HashMap;

pub fn word_frequencies(input: &str) -> std::collections::HashMap<String, usize> {
    let mut counter: HashMap<String, usize> = HashMap::new();
    let words = input.split_whitespace();
    for s in words {
        let temp = s.to_string();
        let count = counter.entry(temp).or_insert(0);
        *count += 1;
    }
    counter
}
