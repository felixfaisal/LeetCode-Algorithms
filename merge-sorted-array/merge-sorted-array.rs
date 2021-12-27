use std::convert::TryInto;
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut nums3: Vec<i32> = vec![0; (n+m).try_into().unwrap()]; 
        let mut i: usize = 0; 
        let mut j: usize = 0; 
        let mut k: usize = 0; 
        if m==0 {
            *nums1 = nums2.clone();
            return
        }
        while i<m.try_into().unwrap() && j<n.try_into().unwrap(){
            if nums1[i] <= nums2[j] {
                nums3[k] = nums1[i]; 
                k+=1; 
                i+=1; 
            }
            else{
                nums3[k] = nums2[j]; 
                k+=1; 
                j+=1; 
            }
        }
        while i<m.try_into().unwrap(){
            nums3[k] = nums1[i]; 
            i+=1; 
            k+=1; 
        }
        
        while j<n.try_into().unwrap(){
            nums3[k] = nums2[j]; 
            j+=1; 
            k+=1; 
        }
        *nums1 = nums3.clone()
    }
}