impl Solution {
    pub fn find_lu_slength(a: String, b: String) -> i32 {
        if a.len() != b.len() {
            return a.len().max(b.len()) as i32;
        }
        
        if a == b {
            return -1;
        } else {
            return a.len() as i32;
        }
    }
}