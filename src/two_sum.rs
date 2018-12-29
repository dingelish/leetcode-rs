// 1 Two Sum
use Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;

        let mut all_num: HashMap<i32,i32> = HashMap::new();

        for (i, item) in nums.iter().enumerate() {
            all_num.insert(*item, i as i32);
        }

        for (i, item) in nums.iter().enumerate() {
            if let Some(x) = all_num.get(&(target - *item)) {
                if *x != i as i32 {
                    return vec![i as i32, *x];
                }
            }
        }

        unreachable!()
    }
}
