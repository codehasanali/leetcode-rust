/**
 * [5] Longest Palindromic Substring
 *
 * Given a string s, return the longest <span data-keyword="palindromic-string">palindromic</span> <span data-keyword="substring-nonempty">substring</span> in s.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: s = "babad"
 * Output: "bab"
 * Explanation: "aba" is also a valid answer.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: s = "cbbd"
 * Output: "bb"
 * 
 *  
 * Constraints:
 * 
 * 	1 <= s.length <= 1000
 * 	s consist of only digits and English letters.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-palindromic-substring/
// discuss: https://leetcode.com/problems/longest-palindromic-substring/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut answer: String = String::new();
        let mut s: Vec<char> = s.chars().collect();
        for i in 0..s.len() {
            for j in i + answer.len()..s.len() {
                if s[i..=j].len() > answer.len() &&
                   s[i..=j].iter().rev().eq(s[i..=j].iter())
                {
                    answer = s[i..=j].iter().collect();
                }
            }
        }
        answer
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_5() {
        assert_eq!(Solution::longest_palindrome(String::from("babad")), "bab".to_string());
        assert_eq!(Solution::longest_palindrome(String::from("cbbd")), "bb".to_string());
        assert_eq!(Solution::longest_palindrome(String::from("a")), "a".to_string());
        assert_eq!(Solution::longest_palindrome(String::from("ac")), "a".to_string());
        assert_eq!(Solution::longest_palindrome(String::from("racecar")), "racecar".to_string());
    }
}
