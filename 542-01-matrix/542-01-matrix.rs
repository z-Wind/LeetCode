impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = mat.len();
        let n = mat[0].len();
        let mut dis_mat = vec![vec![-1; n]; m];

        for i in 0..m {
            for j in 0..n {
                let mut dis = i32::MAX;
                dis_nearest_zero(i, j, &mut dis_mat, &mat, 0, &mut dis);
                dis_mat[i][j] = dis;
            }
        }

        dis_mat
    }
}

fn dis_nearest_zero(i: usize, j: usize, dis_mat: &mut Vec<Vec<i32>>, mat: &Vec<Vec<i32>>, tmp:i32, dis: &mut i32) {
    let m = mat.len();
    let n = mat[0].len();
    
    if tmp >= *dis || i >= m || j >= n || dis_mat[i][j] == -2 {
        return;
    } else if mat[i][j] == 0 {
        *dis = (*dis).min(tmp);
        return;
    } else if dis_mat[i][j] >= 0 {
        *dis = (*dis).min(tmp + dis_mat[i][j]);
        return;
    }
    // tag visited
    dis_mat[i][j] = -2;

    dis_nearest_zero(i - 1, j, dis_mat, mat, tmp + mat[i][j], dis);
    dis_nearest_zero(i, j - 1, dis_mat, mat, tmp + mat[i][j], dis);
    dis_nearest_zero(i + 1, j, dis_mat, mat, tmp + mat[i][j], dis);
    dis_nearest_zero(i, j + 1, dis_mat, mat, tmp + mat[i][j], dis);
    
    dis_mat[i][j] = -1;
}
