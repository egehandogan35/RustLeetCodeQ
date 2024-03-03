impl Solution {
    pub fn maximum_odd_binary_number(s: String) -> String {
        let mut result: String = String::new();
        let mut ones = 0;
        let mut zero = 0;

        for c in s.chars() {
            if c == '1' {
                ones += 1;
            } else {
                zero += 1;
            }
        }

        while ones > 1 {
            result.insert_str(0, "1");
            ones -= 1;
        }

        while zero > 0 {
            result.push('0');
            zero -= 1;
        }
        if ones == 1 {
            result.push('1');
            ones-=1;
        }

        result
    }
}
