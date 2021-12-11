impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = heights.len();
        let n = heights[0].len();

        let mut map: Vec<Vec<i32>> = vec![vec![-1; n]; m];
        let mut ans: Vec<Vec<i32>> = Vec::new();
        for i in 0..m {
            for j in 0..n {
                // println!("");
                // println!("{},{}==========================",i,j);
                map[i][j] = search(&mut map, &heights, i, j);
                // println!("{:02b}=============================",map[i][j]);
                if map[i][j] == 0b11 {
                    ans.push(vec![i as i32, j as i32]);
                }
            }
        }

        // println!("{:?}", map);
        ans
    }
}

fn search(map: &mut Vec<Vec<i32>>, heights: &Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
    if map[i][j] >= 0 {
        return map[i][j];
    }
    if map[i][j] == -2{
        return 0;
    }
    map[i][j] = -2;
    
    let m = heights.len();
    let n = heights[0].len();
    let mut ans = 0;
    // Pacific
    if i == 0 || j == 0 {
        ans |= 0b10;
    }
    // Atlantic
    if i == m - 1 || j == n - 1 {
        ans |= 0b01;
    }

    // up
    if ans != 0b11 && i - 1 < m && heights[i][j] >= heights[i - 1][j] {
        ans |= search(map, heights, i - 1, j);
    }
    // down
    if ans != 0b11 && i + 1 < m && heights[i][j] >= heights[i + 1][j] {
        ans |= search(map, heights, i + 1, j);
    }
    // left
    if ans != 0b11 && j - 1 < n && heights[i][j] >= heights[i][j - 1] {
        ans |= search(map, heights, i, j - 1);
    }
    // right
    if ans != 0b11 && j + 1 < n && heights[i][j] >= heights[i][j + 1] {
        ans |= search(map, heights, i, j + 1);
    }
    
    map[i][j] = -1;
    // println!("{},{}=>{:02b}",i,j,ans);
    ans
}
