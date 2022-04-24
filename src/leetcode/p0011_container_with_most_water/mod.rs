pub fn max_area(height: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = height.len() - 1;
    let mut max = 0;

    while left < right {
        let curr_area = height[left].min(height[right]) * (right - left) as i32;
        max = max.max(curr_area);
        if height[left] < height[right] {
            left+=1;
        } else {
            right-=1;
        }
    }

    max
}

pub fn max_area_bruces_force(height: Vec<i32>) -> i32 {
    let mut max = 0;
    let len = height.len();
    for (i, value) in height.iter().enumerate() {
        for j in 1..len {
            let curr = (*value).min(height[j]) * (j as i32 - i as i32);
            max = max.max(curr);
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use crate::leetcode::p0011_container_with_most_water::{max_area, max_area_bruces_force};
    #[test]
    pub fn test() {
        assert_eq!(49, max_area(vec![1,8,6,2,5,4,8,3,7]));
        assert_eq!(1, max_area(vec![1, 1]));

        assert_eq!(49, max_area_bruces_force(vec![1,8,6,2,5,4,8,3,7]));
        assert_eq!(1, max_area_bruces_force(vec![1, 1]));

    }
}