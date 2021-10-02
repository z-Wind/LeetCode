impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let mut h = citations.len() as i32;
        let mut count = 0;
        for &c in citations.iter().rev(){
            if c < h{
                while h > c{
                    h-=1;
                    if count >= h{
                        return h;
                    }        
                }
            }
            count += 1;
            // println!("{} => h:{} count:{}", c, h, count);
            
            if count >= h{
                return h;
            }
        }
        0
    }
}