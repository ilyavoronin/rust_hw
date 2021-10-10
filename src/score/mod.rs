use std::collections::HashMap;

pub fn transform(old: &HashMap<u32, Vec<char>>) -> HashMap<char, u32> {
    let mut result = HashMap::new();
    old.iter().for_each(|(num, chars)| {
        chars.iter().for_each(|c| {
            result.insert(c.to_ascii_lowercase(), *num);
        })
    });
    result
}

pub fn score(input: &str, score_table: &HashMap<char, u32>) -> u32 {
    input.chars().into_iter().map(|c|
        score_table.get(&c.to_ascii_lowercase()).unwrap_or(&0)
    ).sum()
}
