// 0 -> 1
// 0_0 0_1 -> 1_1 1_0
// 0_00 0_01 0_11 0_10 -> 1_10 1_11 1_01 1_00 
impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        let mut ans:Vec<i32> = vec![0];
        for b in (0..n){
            //println!("{}: {:?}", b, ans);
            for i in (0..ans.len()).rev(){
                ans.push(ans[i] | 1<<b);
            }
        }
        ans
    }
}

