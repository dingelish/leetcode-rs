// 3. Longest Substring Without Repeating Characters

use crate::Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut left: usize = 0;
        let mut ans: usize = 0;
        let mut arr: [i32; 256] = [-1; 256];

        for (i, c) in s.chars().enumerate() {
            if arr[c as usize] >= left as i32 {
                left = (arr[c as usize] + 1) as usize;
            }
            arr[c as usize] = i as i32;
            ans = std::cmp::max(ans, i + 1 - left);
        }

        ans as i32
    }
}
