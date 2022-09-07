impl Solution {
    pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        let m = mat.len();
        let n = mat[0].len();
        let r = r as usize;
        let c = c as usize;

        if (m == r && r == c) || (m * n != r * c) {
            return mat;
        }

        let mut data = mat.into_iter().flatten();
        let mut result: Vec<Vec<i32>> = Vec::with_capacity(r);
        for i in 0..r {
            let mut v = Vec::with_capacity(c);
            for j in 0..c {
                v.push(data.next().expect("number"));
            }
            result.push(v);
        }

        result
    }
}
