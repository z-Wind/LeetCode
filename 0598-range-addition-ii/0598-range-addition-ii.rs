impl Solution {
    pub fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
        let range = ops.iter().fold(vec![m, n], |acc, x| {
            vec![acc[0].min(x[0]), acc[1].min(x[1])]
        });

        range[0] * range[1]
    }
}
