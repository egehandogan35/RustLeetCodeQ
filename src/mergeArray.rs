fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &Vec<i32>, n: i32) { //merge sorted arrays q88
    let mut nums1_pointer = m - 1; //mutable array pointer in nums1
    let mut nums2_pointer = n - 1; //mutable array pointer in nums2
    let mut merged_pointer = m + n - 1; //mutable pointer in merged

    while nums1_pointer >= 0 && nums2_pointer >= 0 {
        if nums1[nums1_pointer as usize] > nums2[nums2_pointer as usize] { //find larger element
            nums1[merged_pointer as usize] = nums1[nums1_pointer as usize]; //copy to p position
            nums1_pointer -= 1;
        } else { //smaller or equal
            nums1[merged_pointer as usize] = nums2[nums2_pointer as usize];
            nums2_pointer -= 1;
        }
        merged_pointer -= 1;
    }
    while nums2_pointer >= 0 { // remaining elements
        nums1[merged_pointer as usize] = nums2[nums2_pointer as usize];
        nums2_pointer -= 1;
        merged_pointer -= 1;
    }
}
