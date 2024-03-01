impl Solution {
pub fn is_match(s: String, p: String) -> bool {
    let s_chars: Vec<char> = s.chars().collect();
    let p_chars: Vec<char> = p.chars().collect();
    let mut dp = vec![vec![false; p.len() + 1]; s.len() + 1];
    dp[0][0] = true;
    for j in 2..=p.len() {
        if p_chars[j - 1] == '*' {
            dp[0][j] = dp[0][j - 2];
        }
    }
    for i in 1..=s.len() {
        for j in 1..=p.len() {
            if p_chars[j - 1] == '.' || p_chars[j - 1] == s_chars[i - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else if p_chars[j - 1] == '*' {
                dp[i][j] = dp[i][j - 2];
                if p_chars[j - 2] == '.' || p_chars[j - 2] == s_chars[i - 1] {
                    dp[i][j] = dp[i][j] || dp[i - 1][j];
                }
            }
        }
    }
    dp[s.len()][p.len()]
}
}
