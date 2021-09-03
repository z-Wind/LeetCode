impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        match row_index{
            0 => vec![1],
            x => {
                let mut ans = Self::get_row(x-1);
                let mut temp:Vec<i32> = vec![1];
                for w in ans.windows(2){
                    temp.push(w[0]+w[1]);
                }
                temp.push(1);
                temp
            },
        }
    }
}