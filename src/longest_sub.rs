impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut char_indices = std::collections::HashMap::new();
        let mut start = 0;
        let mut longest_length = 0;

        for (end, (first_iter, second_iter)) in s
            .chars()
            .zip(s.chars().chain(std::iter::once(' ')))
            .enumerate()
        {
            if let Some(exist) = char_indices.get(&second_iter).copied() {
                start = start.max(exist + 1);
            }
            char_indices.insert(second_iter, end);
            longest_length = longest_length.max(end - start + 1);
        }

        longest_length as i32
    }
}
