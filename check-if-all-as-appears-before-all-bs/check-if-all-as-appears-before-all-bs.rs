impl Solution {
    pub fn check_string(s: String) -> bool {
        let s_vec: Vec<char> = s.chars().collect();
        for x in 1..s_vec.len(){
            if s_vec[x-1] == 'b' && s_vec[x] == 'a'{
                return false;
            }
        }
        true
    }
}