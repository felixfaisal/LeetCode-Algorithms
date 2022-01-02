impl Solution {
    pub fn asteroids_destroyed(mass: i32, asteroids: Vec<i32>) -> bool {
        let mut s_ast = asteroids.clone(); 
        s_ast.sort(); 
        let mut m_mass = mass; 
        for x in 0..s_ast.len(){
            if m_mass >= 100000 { return true ;}
            if m_mass < s_ast[x] { return false; }
            m_mass += s_ast[x];
        }
        true
    }
}