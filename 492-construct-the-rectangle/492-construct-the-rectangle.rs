impl Solution {
    pub fn construct_rectangle(area: i32) -> Vec<i32> {
        let mut ans = vec![i32::MAX, 0];
        for L in (1..=area).rev() {
           if area % L == 0 {
               let W = area / L;
               if W > L {
                   break;
               }
               ans = vec![L, W];
            } 
        }
        
        ans
    }
}