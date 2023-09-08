impl Solution {
    pub fn outer_trees(trees: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = trees.len();
        if n <= 3 {
            return trees;
        }

        let mut trees = trees;
        trees.sort_unstable_by_key(|x| x[0]);
        // println!("{:?}", trees);

        let mut used = vec![0; n];

        let mut result = vec![trees[0].clone()];
        used[0] = 1;
        let mut i = 0;
        loop {
            let mut j = match used.iter().position(|&x| x == 0) {
                Some(x) => x,
                None => break,
            };
            let mut cross = 0;
            for k in 0..n {
                if k == i || k == j || used[k] == 1 {
                    continue;
                }
                let a = (trees[j][0] - trees[i][0], trees[j][1] - trees[i][1]);
                let b = (trees[k][0] - trees[i][0], trees[k][1] - trees[i][1]);
                cross = cross_product(b, a);
                if cross > 0 || (cross == 0 && a.0 * a.0 + a.1 * a.1 > b.0 * b.0 + b.1 * b.1) {
                    j = k;
                }
            }
            if i != 0 {
                let a = (trees[j][0] - trees[i][0], trees[j][1] - trees[i][1]);
                let b = (trees[0][0] - trees[i][0], trees[0][1] - trees[i][1]);
                cross = cross_product(b, a);
                if cross > 0 {
                    j = 0;
                }
            }
            // println!("{:?} to {:?}", trees[i], trees[j]);
            if j == 0 {
                break;
            }
            result.push(trees[j].clone());
            used[j] = 1;
            i = j;
        }
        result
    }
}

// https://en.wikipedia.org/wiki/Cross_product
fn cross_product(a: (i32, i32), b: (i32, i32)) -> i32 {
    a.0 * b.1 - b.0 * a.1
}
