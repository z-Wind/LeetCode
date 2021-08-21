// C(n,k)=C(n-1,k-1)+C(n-1,k)
// https://zhidao.baidu.com/question/368034364206885564.html
use std::collections::HashMap;
impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut c = Combine::new();
        c.combine(n,k)
    }
}

struct Combine{
    dp:HashMap<(i32,i32),Vec<Vec<i32>>>,
}

impl Combine{
    fn new() -> Self{
        Combine{
            dp: HashMap::new(),
        }
    }
    fn combine(&mut self, n: i32, k: i32) -> Vec<Vec<i32>>{
        match self.dp.get(&(n,k)){
            Some(v) => v.to_vec(),
            None => {
                if k == 0 {
                    return vec![vec![]];
                } else if k == n{
                    return vec![(1..=n).collect()];
                }

                let mut result:Vec<Vec<i32>> = vec![];

                for mut cmb in self.combine(n - 1, k - 1) {
                    cmb.push(n);
                    result.push(cmb);
                }

                result.append(&mut self.combine(n - 1, k));

                self.dp.insert((n,k),result.clone());
                result
            }
        }
    }
}