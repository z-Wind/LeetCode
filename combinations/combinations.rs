// C(n,k)=C(n-1,k-1)+C(n-1,k)
// https://zhidao.baidu.com/question/368034364206885564.html
impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        if k == 0 {
            return vec![vec![]];
        } else if k == n{
            return vec![(1..=n).collect()];
        }
        
        let mut result:Vec<Vec<i32>> = vec![];
        
        for mut cmb in Self::combine(n - 1, k - 1) {
            cmb.push(n);
            result.push(cmb);
        }
        
        result.append(&mut Self::combine(n - 1, k));
        
        result
    }
}
