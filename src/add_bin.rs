impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut a = a.chars().rev().collect::<Vec<_>>();
        let mut b = b.chars().rev().collect::<Vec<_>>();
        let mut carry = 0;
        let mut result = Vec::new();

        while a.len() < b.len() { a.push('0'); }
        while b.len() < a.len() { b.push('0'); }

        for (a, b) in a.iter().zip(b.iter()) {
            let sum = a.to_digit(10).unwrap() + b.to_digit(10).unwrap() + carry;
            result.push(std::char::from_digit(sum % 2, 10).unwrap());
            carry = sum / 2;
        }

        if carry != 0 {
            result.push('1');
        }

        result.iter().rev().collect()
    }
}
