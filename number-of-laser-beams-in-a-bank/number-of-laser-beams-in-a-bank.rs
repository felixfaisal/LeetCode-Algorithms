impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
       let mut prev = 0; 
        let mut laser = 0; 
        for i in bank{
            let mut curr = 0;
            for x in i.chars(){
                if x == '1' {
                    curr += 1; 
                }
            }
            laser += prev*curr; 
            if curr != 0 {
                prev = curr;
            }
            
        }
        laser
    }
}