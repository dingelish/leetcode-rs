// 4. Median of Two Sorted Arrays
use crate::Solution;

impl Solution {
    fn find_kth(n1: &[i32], n2: &[i32], k: usize) -> f64 {
        if n1.is_empty() {
            return f64::from(n2[k - 1]);
        }

        if n2.is_empty() {
            return f64::from(n1[k - 1]);
        }

        if k == 1 {
            return f64::from(std::cmp::min(n1[0], n2[0]));
        }

        let a = if n1.len() >= k / 2 {
            n1[k / 2 - 1]
        } else {
            std::i32::MAX
        };

        let b = if n2.len() >= k / 2 {
            n2[k / 2 - 1]
        } else {
            std::i32::MAX
        };

        if a < b {
            Solution::find_kth(&n1[k / 2..], n2, k - k / 2)
        } else {
            Solution::find_kth(n1, &n2[k / 2..], k - k / 2)
        }
    }

    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let n = nums1.len() + nums2.len();

        if n % 2 == 1 {
            Solution::find_kth(&nums1, &nums2, n / 2 + 1)
        } else {
            let k1 = Solution::find_kth(&nums1, &nums2, n / 2);
            let k2 = Solution::find_kth(&nums1, &nums2, n / 2 + 1);

            (k1 + k2) as f64 / 2f64
        }
    }
}
