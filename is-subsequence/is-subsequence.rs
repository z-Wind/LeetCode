impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut t = t.chars();
        for c1 in s.chars(){
            loop{
                match t.next(){
                    Some(c2) => {
                        if c1 == c2{
                            break;
                        }
                    },
                    None => {
                        return false;
                    }
                }
            }
        }
        true
    }
}