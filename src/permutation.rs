use std::collections::HashMap;

pub fn are_permutations(strs: Vec<String>) -> bool {
    if strs.len() < 2 {
        return false;
    }
    let first_map = build_permutation_map(&strs[0]);
    for s in &strs[1..] {
        let next_map = build_permutation_map(s);
        if next_map != first_map {
            return false;
        }
    }
    return true;
}

fn build_permutation_map(s: &str) -> HashMap<char, usize> {
    let mut map = HashMap::new();
    for c in s.chars() {
        let existing_value = map.get(&c);
        map.insert(c, existing_value.unwrap_or(&0) + 1);
    }
    return map;
}
