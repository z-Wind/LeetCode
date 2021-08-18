impl Solution {
    pub fn is_number(s: String) -> bool {
        if s.contains("inf"){
            return false;
        }
        
        s.parse::<f64>().is_ok()
    }
}