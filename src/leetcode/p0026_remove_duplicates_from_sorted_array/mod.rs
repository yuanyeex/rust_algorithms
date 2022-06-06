pub fn remove_dulicates(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }
    let mut curr_ind = 0;
    for i in 1..nums.len() {
        if nums[curr_ind] != nums[i] {
            let tmp = nums[i];
            curr_ind += 1;
            nums[curr_ind] = tmp;
        }
    }
    (curr_ind + 1) as i32
}

#[cfg(test)]
mod tests {
    use crate::leetcode::p0026_remove_duplicates_from_sorted_array::remove_dulicates;

    #[test]
    pub fn test() {
        assert_eq!(5, remove_dulicates(&mut vec![0,0,1,1,1,2,2,3,3,4]));
        assert_eq!(2, remove_dulicates(&mut vec![1,1,2]));
    }
}