use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut cache: HashMap<i32, i32> = HashMap::new();
    for (index, elem) in nums.iter().enumerate() {
        if let Some(value) = cache.get(elem) {
            return vec![*value, index as i32];
        } else {
            cache.insert(target - elem, index as i32);
        }
    }
    vec![]
}

#[cfg(test)]
mod test {
    use crate::leetcode::two_sum;

    #[test]
    fn test_two_sum() {
        let nums = vec![2, 7, 11, 15];
        assert_eq!(two_sum(nums, 9), vec![0, 1]);

        let nums = vec![3, 2, 4];
        assert_eq!(two_sum(nums, 6), vec![1, 2]);

        let nums = vec![3, 3];
        assert_eq!(two_sum(nums, 6), vec![0, 1]);
    }
}
