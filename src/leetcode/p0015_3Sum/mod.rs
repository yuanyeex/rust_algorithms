pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {

    let mut nums = nums;
    nums.sort();

    let mut res: Vec<Vec<i32>> = vec![];
    // first index, for 0 ... sortedNums.len
    for i in 0..nums.len() {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }
        let (mut l, mut r) = (i + 1, nums.len() - 1);
        while l < r {
            let left = nums[l];
            let right = nums[r];
            if l > i + 1 && left == nums[l -1] {
                l+=1;
                continue;
            }
            if r < nums.len() - 1 && right == nums[r + 1] {
                r -= 1;
                continue;
            }

            let num = nums[i];
            let total = left + right +num;
            if total == 0 {
                res.push([num, left, right].to_vec());
                l += 1;
            } else if total < 0 {
                l += 1;
            } else {
                r -= 1;
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use crate::leetcode::p0015_3Sum::three_sum;

    #[test]
    pub fn test() {
        let res = three_sum(vec![-1, 0, 1 ,2 ,-1, 4]);
        println!("res {:?}", res);
        assert_eq!(res, vec![vec![-1,-1, 2], vec![-1, 0, 1]]);
    }

}