impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let n = n as usize + 1;
        let mut ans = Vec::with_capacity(n as usize + 1);
        ans.push(0);
        let mut i = 1;
        while i < n{
            if i&1 == 0{
                for j in 0..i{
                    ans.push(ans[j]+1);
                    i+=1;
                }
            } else {
                ans.push(ans[i-1]+1);
                i+=1;
            }
            // println!("{}:{:?}", i, ans);
        }
        ans[..n].to_vec()
    }
}