impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut result = digits.clone();
    let mut carry = 1;

    for i in (0..result.len()).rev() {
        result[i] += carry;
        carry = result[i] / 10;
        result[i] %= 10;
    }

    if carry > 0 {
        result.insert(0, carry);
    }

    result
    }
}
