pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut nums = nums;
    nums.sort();

    let mut res: Vec<Vec<i32>> = vec![];
    let avg_four = target / 4;
    let avg_two = target / 2;

    let nums_len = nums.len();
    for (i, num) in nums.iter().enumerate() {
        // first round
        // skip duplicates
        if i > 0 && nums[i - 1] == *num {
            continue;
        }

        if *num > avg_four {
            // finish fast
            return res;
        }

        let target_3 = target - num;
        // second round
        for k in i + 1..nums_len {
            // skip duplicates
            if k > i + 1 && nums[k] == nums[k - 1] {
                continue;
            }

            if *num + nums[k] > avg_two {
                // finish fast
                continue;
            }
            let target_2 = target_3 - nums[k];
            let mut l = k + 1;
            let mut r = nums_len - 1;
            while l < r {
                if l > k + 1 && nums[l] == nums[l -1] {
                    l += 1;
                    continue;
                }
                if r < nums_len - 1 && nums[r] == nums[r + 1] {
                    r -= 1;
                    continue;
                }
                let sum_2 = nums[l] + nums[r];
                if target_2 == sum_2 {
                    res.push(vec![*num, nums[k], nums[l], nums[r]]);
                    l += 1;
                } else if sum_2 > target_2 {
                    r -= 1;
                } else {
                    l += 1;
                }
            }
        }
    }
    res
}

#[cfg(test)]
mod tests{
    use crate::leetcode::p0018_4sum::four_sum;

    /// Input: nums = [1,0,-1,0,-2,2], target = 0
    // Output: [[-2,-1,1,2],[-2,0,0,2],[-1,0,0,1]]
    #[test]
    fn test() {
        assert_eq!(vec![vec![-2,-1,1,2],vec![-2,0,0,2],vec![-1,0,0,1]], four_sum(vec![1,0,-1,0,-2,2], 0));
        assert_eq!(vec![vec![2,2,2,2]], four_sum(vec![2,2,2,2,2], 8));
    }
}