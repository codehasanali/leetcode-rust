use std::collections::HashMap;

/**
 * [3] Longest Substring Without Repeating Characters
 *
 * Given a string s, find the length of the longest <span data-keyword="substring-nonempty">substring</span> without repeating characters.
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "abcabcbb"
 * Output: 3
 * Explanation: The answer is "abc", with the length of 3.
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "bbbbb"
 * Output: 1
 * Explanation: The answer is "b", with the length of 1.
 *
 * <strong class="example">Example 3:
 *
 * Input: s = "pwwkew"
 * Output: 3
 * Explanation: The answer is "wke", with the length of 3.
 * Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.
 *
 *  
 * Constraints:
 *
 * 	0 <= s.length <= 5 * 10^4
 * 	s consists of English letters, digits, symbols and spaces.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-substring-without-repeating-characters/
// discuss: https://leetcode.com/problems/longest-substring-without-repeating-characters/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut index_map: HashMap<char, i32> = HashMap::new();
        let mut max_len = 0;
        let mut start = 0;
        let mut curr = 0;

        s.chars().for_each(|c| {
            index_map.entry(c).and_modify(|prev| {
                let len = curr - start;
                if len > max_len {
                    max_len = len;
                }
                if *prev >= start {
                    start = *prev + 1;
                }
                *prev = curr;
            }).or_insert(curr);
            curr += 1;
        });
        curr -= start;
        if curr > max_len { curr } else { max_len }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3() {
        assert_eq!(Solution::length_of_longest_substring("abcabcbb".to_string()), 3);
        assert_eq!(Solution::length_of_longest_substring("bbbbb".to_string()), 1);
        assert_eq!(Solution::length_of_longest_substring("pwwkew".to_string()), 3);
        assert_eq!(Solution::length_of_longest_substring("".to_string()), 0);
        assert_eq!(Solution::length_of_longest_substring("a".to_string()), 1);
        assert_eq!(Solution::length_of_longest_substring("au".to_string()), 2);
        assert_eq!(Solution::length_of_longest_substring("aab".to_string()), 2);
        assert_eq!(Solution::length_of_longest_substring("dvdf".to_string()), 3);
    
    }
}
