impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut lo: i32 = 0; 
        let mut hi: i32 = (nums.len()-1) as i32; 
        while lo<=hi {
            let mid: i32 = (lo+hi)/2; 
            if nums[mid as usize] == target {
                return mid as i32; 
            }
            else if nums[mid as usize] < target {
                lo = mid+1; 
            }
            else {
                hi = mid-1; 
            }
        }
        return lo;
    }
}