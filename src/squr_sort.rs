impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() == 1 {
            return vec![nums[0] * nums[0]];
        }

        let mut result: Vec<i32> = Vec::new();
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left < right {
            let left_square = nums[left] * nums[left];
            let right_square = nums[right] * nums[right];

            if left_square > right_square {
                result.push(left_square);
                left += 1;
            } else {
                result.push(right_square);
                right -= 1;
            }
        }
        result.push(nums[left] * nums[left]);
        result.reverse();
        result
    }
}
