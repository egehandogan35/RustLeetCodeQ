impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut left = 0;
        let mut right = x;
        let mut result = 0;
        while left <= right {
            let mid = left + (right - left) / 2;
            if mid as i64 * mid as i64 <= x as i64 {
                result = mid;
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        result
    }
}
