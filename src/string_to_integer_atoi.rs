impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut iter_s = s.trim().chars().peekable();
        let mut result: i128 = 0;
        let mut sign = 1;
        if let Some(&c) = iter_s.peek() {
            if c == '-' {
                sign = -1;
                iter_s.next();
            } else if c == '+' {
                iter_s.next();
            }
        }
        while let Some(&c) = iter_s.peek() {
            if !c.is_digit(10) {
                break;
            }
            if let Some(next_digit) = c.to_digit(10) {
                result = result
                    .checked_mul(10)
                    .and_then(|r| r.checked_add(next_digit as i128))
                    .unwrap_or_else(|| if sign == 1 { i32::MAX as i128 } else { i32::MIN as i128 });
                iter_s.next();
            }
        }
        result *= sign;
        if result > i32::MAX as i128 {
            i32::MAX
        } else if result < i32::MIN as i128 {
            i32::MIN
        } else {
            result as i32
        }
    }
}
