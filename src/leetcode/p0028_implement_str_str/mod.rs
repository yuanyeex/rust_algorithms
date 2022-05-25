pub fn str_str(haystack: String, needle: String) -> i32 {
    if needle.is_empty() {
        return 0;
    }
    if let Some(ind) = haystack.find(&needle) {
        ind as i32
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::p0028_implement_str_str::str_str;

    #[test]
    pub fn test() {
        assert_eq!(0, str_str("fddf".to_string(), "".to_string()));
        assert_eq!(1, str_str(String::from("xfdfd"), String::from("fd")))
    }
}