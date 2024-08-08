/**
 * [354] Russian Doll Envelopes
 *
 * You are given a 2D array of integers envelopes where envelopes[i] = [wi, hi] represents the width and the height of an envelope.
 * One envelope can fit into another if and only if both the width and height of one envelope are greater than the other envelope's width and height.
 * Return the maximum number of envelopes you can Russian doll (i.e., put one inside the other).
 * Note: You cannot rotate an envelope.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: envelopes = [[5,4],[6,4],[6,7],[2,3]]
 * Output: 3
 * Explanation: The maximum number of envelopes you can Russian doll is 3 ([2,3] => [5,4] => [6,7]).
 * 
 * <strong class="example">Example 2:
 * 
 * Input: envelopes = [[1,1],[1,1],[1,1]]
 * Output: 1
 * 
 *  
 * Constraints:
 * 
 * 	1 <= envelopes.length <= 10^5
 * 	envelopes[i].length == 2
 * 	1 <= wi, hi <= 10^5
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/russian-doll-envelopes/
// discuss: https://leetcode.com/problems/russian-doll-envelopes/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_envelopes(envelopes: Vec<Vec<i32>>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_354() {
    }
}
