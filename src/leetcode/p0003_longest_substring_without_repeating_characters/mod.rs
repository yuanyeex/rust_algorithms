use std::collections::HashMap;

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut index_cache = HashMap::new();
    let mut start_ind = 0;
    let mut longest = 0;
    for (ind, ch) in s.chars().enumerate() {
        if let Some(prev) = index_cache.get(&ch) {
            start_ind = start_ind.max(*prev);
        }
        longest = longest.max(ind + 1 - start_ind);
        index_cache.insert(ch, ind + 1);
    }

    longest as i32
}

#[cfg(test)]
mod tests {
    use super::length_of_longest_substring;

    #[test]
    fn test() {
        assert_eq!(3, length_of_longest_substring("abcabcbb".to_owned()));
        assert_eq!(1, length_of_longest_substring("bbbbb".to_owned()));
        assert_eq!(3, length_of_longest_substring("pwwkew".to_owned()));
        //sucks 
        assert_eq!(3, length_of_longest_substring("dvdf".to_owned()));
        assert_eq!(1, length_of_longest_substring("s".to_owned()));
    }
}