impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() as i32 - 1;

        while left <= right {
            let mid = left + (right - left) / 2;
            let mid_value = nums[mid as usize];

            if mid_value == target {
                return mid;
            } else if mid_value < target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        left
    }
}
