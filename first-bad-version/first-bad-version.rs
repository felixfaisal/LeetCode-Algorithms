// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
	let mut left = 1;
	let mut right = n;

	while left != right {
		let middle = left + (right - left) / 2;
		if self.isBadVersion(middle) {
			right = middle;
		} else {
			left = middle + 1;
		}
	}

	left // Guaranteed to be bad.
    }
}
/*
n=5, bad = 4 
1-5 
mid(3) = false 
mid(5) = true 
mid(4) = true 

1 2 3 4 5 

4/2 = 2 
2+1+4/2 = 7/2 = 4 = nums[4] = true 
Basically if false -> go right 
if true -> go left 
if a[mid] == true && a[mid-1] == false return  
*/