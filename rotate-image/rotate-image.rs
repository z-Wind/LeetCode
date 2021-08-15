impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        matrix.reverse();
        
        for i in 0..matrix.len() {
            for j in 0..i {
                mat_swap_split(matrix, i,j, j,i);
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