impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if let Some(first_str) = strs.get(0) {
            let mut same_chars = String::new();

            for (i, char1) in first_str.chars().enumerate() {
                if strs.iter().skip(1).all(|s| s.chars().nth(i) == Some(char1)) {
                    same_chars.push(char1);
                } else {
                    break;
                }
            }

            same_chars
        } else {
            String::new()
        }
    }
}
