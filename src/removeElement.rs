pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut i = 0;
     let mut j = 0;
 
     while i < nums.len() {
         if nums[i] == val {
             i += 1;
         } else {
             nums[j] = nums[i];
             i += 1;
             j += 1;
         }
     }
 
     nums.truncate(j);
 
     nums.len() as i32
    }