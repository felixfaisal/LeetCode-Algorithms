impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; nums.len()];
        for i in 0..nums.len() {
            res[i] = nums[i] * nums[i];
        }
        res.sort_unstable();
        res
    }
}