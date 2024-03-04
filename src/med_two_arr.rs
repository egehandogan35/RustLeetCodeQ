impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut concat_vec: Vec<_> = nums1.iter().cloned().chain(nums2.iter().cloned()).collect();
        concat_vec.sort();

        let length = concat_vec.len();

        match length {
            0 => 0.0,
            _ => {
                let middle = length / 2;
                if length % 2 == 0 {
                    (concat_vec[middle - 1] as f64 + concat_vec[middle] as f64) / 2.0
                } else {
                    concat_vec[middle] as f64
                }
            }
        }
    }
}
