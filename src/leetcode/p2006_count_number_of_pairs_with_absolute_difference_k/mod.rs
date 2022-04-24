use std::collections::HashMap;

pub fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut total = 0;
    for value in nums.iter() {
        if let Some(v) = map.get(&(value - k)) {
            total += v;
        }
        if let Some(v) = map.get(&(value + k)) {
            total += v;
        }
        map.entry(*value).and_modify(|e| *e += 1).or_insert(1);
    }
    total
}

#[cfg(test)]
mod tests {
    use super::count_k_difference;
    #[test]
    pub fn test() {
        assert_eq!(4, count_k_difference(vec![1, 2, 2, 1], 1));
        assert_eq!(0, count_k_difference(vec![1, 3], 3));
    }
}
