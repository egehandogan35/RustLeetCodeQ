impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let mut temp_num = x;
        let mut num_digits = 0;
        while temp_num > 0 {
            temp_num /= 10;
            num_digits += 1;
        }

      
        let mut temp_num = x;
        let mut rev_order = 0;
        for position in 0..num_digits {
            let digit = (temp_num / 10i32.pow((num_digits - position - 1) as u32)) % 10;
            rev_order += digit * 10i32.pow(position as u32);

            temp_num %= 10i32.pow((num_digits - position - 1) as u32);
        }

       
        x == rev_order
    }
}
