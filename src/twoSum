impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
       let mut num_index = std::collections::HashMap::new();

        for (index, &num) in nums.iter().enumerate() {
            let remain = target - num;

            if let Some(&remain_index) = num_index.get(&remain) {
                return vec![remain_index as i32, index as i32];
            }

            num_index.insert(num, index);
        }
        Vec::new()

    }
}
