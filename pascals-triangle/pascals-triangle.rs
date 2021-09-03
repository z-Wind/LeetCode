impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        match num_rows{
            1 => vec![vec![1]],
            x => {
                let mut ans = Self::generate(x-1);
                let mut temp:Vec<i32> = vec![1];
                for w in ans.last().unwrap().windows(2){
                    temp.push(w[0]+w[1]);
                }
                temp.push(1);
                ans.push(temp);
                ans
            },
        }
    }
}