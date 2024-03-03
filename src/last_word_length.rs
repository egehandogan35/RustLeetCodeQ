impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let words: Vec<&str> = s.split_whitespace().collect();
        let length = words.len();
        if length >=2 {
            let last_word = words[length-1];
            let count :i32 = last_word.chars().count() as i32;
            count
        }
        else {
            let count:i32 = words[0].to_string().chars().count() as i32;
            count
        }
    }
}
