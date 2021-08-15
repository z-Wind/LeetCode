impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        let mut pre_i = 0;
        let mut pre_j = 0;
        for i in (0..n/2+n%2) {
            for j in (0..n/2) {
                pre_i = i;
                pre_j = j;
                let mut rotate_i = pre_j;
                let mut rotate_j = n - 1 - pre_i;
                for _ in (0..4) {
                    mat_swap_split(matrix, i, j, rotate_i, rotate_j);
                    //println!("({},{}) => ({},{}): {:?}",pre_i,pre_j,rotate_i,rotate_j, matrix);
                    pre_i = rotate_i;
                    pre_j = rotate_j;
                    rotate_i = pre_j;
                    rotate_j = n - 1 - pre_i;
                }
            }
        }
    }
}

fn mat_swap_split<T>(v: &mut Vec<Vec<T>>, i1: usize, j1: usize, i2: usize, j2: usize) {
    if i1 == i2 {
        v[i1].swap(j1, j2);
    } else if i1 > i2 {
        mat_swap_split(v, i2, j2, i1, j1);
    } else {
        let (v1, v2) = v.split_at_mut(i2);

        std::mem::swap(&mut v1[i1][j1], &mut v2[0][j2]);
    }
}
