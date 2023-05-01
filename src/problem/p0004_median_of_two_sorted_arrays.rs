/**
 * [4] Median of Two Sorted Arrays
 *
 * Given two sorted arrays nums1 and nums2 of size m and n respectively, return the median of the two sorted arrays.
 * The overall run time complexity should be O(log (m+n)).
 *  
 * <strong class="example">Example 1:
 * 
 * Input: nums1 = [1,3], nums2 = [2]
 * Output: 2.00000
 * Explanation: merged array = [1,2,3] and median is 2.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: nums1 = [1,2], nums2 = [3,4]
 * Output: 2.50000
 * Explanation: merged array = [1,2,3,4] and median is (2 + 3) / 2 = 2.5.
 * 
 *  
 * Constraints:
 * 
 * 	nums1.length == m
 * 	nums2.length == n
 * 	0 <= m <= 1000
 * 	0 <= n <= 1000
 * 	1 <= m + n <= 2000
 * 	-10^6 <= nums1[i], nums2[i] <= 10^6
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/median-of-two-sorted-arrays/
// discuss: https://leetcode.com/problems/median-of-two-sorted-arrays/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut arr: Vec<i32> = Vec::new();
        let (m, n) = (nums1.len(), nums2.len());
        let (mut i, mut j) = (0, 0);

        while i < m && j < n {
            if nums1[i] <= nums2[j] {
                arr.push(nums1[i]);
                i += 1;
            } else {
                arr.push(nums2[j]);
                j += 1;
            }
        }
        while i < m {
            arr.push(nums1[i]);
            i += 1;
        }
        while j < n {
            arr.push(nums2[j]);
            j += 1;
        }
        let len = arr.len();
        if len % 2 == 0 {
            return (arr[len / 2 - 1] + arr[len / 2]) as f64 / 2.0;
        } else {
            return arr[len / 2] as f64;
        }
    }
}
// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_4() {

        assert_eq!(Solution::find_median_sorted_arrays(vec![1,3], vec![2]), 2.00000);
        assert_eq!(Solution::find_median_sorted_arrays(vec![1,2], vec![3,4]), 2.50000);
        assert_eq!(Solution::find_median_sorted_arrays(vec![], vec![1]), 1.00000);
        assert_eq!(Solution::find_median_sorted_arrays(vec![2], vec![]), 2.00000);
        assert_eq!(Solution::find_median_sorted_arrays(vec![1,2,3], vec![4,5,6]), 3.50000);
    }
}
