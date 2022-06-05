
pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
    let mut nums = nums;
    // get sorted
    nums.sort();

    let mut min_dist = i32::MAX;
    let mut res = 0;
    for (i, num) in nums.iter().enumerate() {
        if i > 0 && nums[i - 1] == *num {
            continue;
        }
        let mut l = i + 1;
        let mut r = nums.len() - 1;
        while l < r {
            let sum = num + nums[l] + nums[r];
            let diff = (sum - target).abs();
            // finish fast
            if diff == 0 {
                return sum;
            }
            // update current closet result
            if diff < min_dist {
                min_dist = diff;
                res = sum;
            }
            if sum > target {
                r -= 1;
            } else {
                l += 1;
            }
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use crate::leetcode::p0016_3Sum_Closest::three_sum_closest;

    #[test]
    fn test() {
        // example 1
        assert_eq!(2, three_sum_closest(vec![-1,2,1,-4], 1));
        // example 2
        assert_eq!(0, three_sum_closest(vec![0,0,0], 1));
        assert_eq!(3, three_sum_closest(vec![0,1,2], 3));
    }
}