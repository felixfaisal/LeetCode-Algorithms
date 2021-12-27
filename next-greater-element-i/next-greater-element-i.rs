use std::collections::HashMap; 
impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
     let mut stack: Vec<i32> = vec![]; 
     let mut greater: HashMap<i32,i32> = HashMap::new();
     let mut sol = vec![0;nums1.len()];
     for i in (0..nums2.len()).rev(){
        while stack.len()>0 && stack[stack.len()-1]<nums2[i]{
            stack.pop(); 
        }  
        if stack.len()>0{
            greater.insert(nums2[i],stack[stack.len()-1]); 
            stack.push(nums2[i]);
         }
        else{
            greater.insert(nums2[i],-1);
            stack.push(nums2[i]);
         }
     }
    for i in 0..nums1.len(){
        sol[i] = *greater.get(&nums1[i]).unwrap();    
    }
    sol
    }
}