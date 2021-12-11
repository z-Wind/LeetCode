impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = heights.len();
        let n = heights[0].len();

        let mut map: Vec<Vec<i32>> = vec![vec![0; n]; m];
        
        for i in 0..m{
            search(&mut map, &heights, i32::MIN, i, 0, 0b10);
            search(&mut map, &heights, i32::MIN, i, n-1, 0b01);
        }
        for j in 0..n{
            search(&mut map, &heights, i32::MIN, 0, j, 0b10);
            search(&mut map, &heights, i32::MIN, m-1, j, 0b01);
        }
        // println!("{:?}", map);
        
        let mut ans: Vec<Vec<i32>> = Vec::new();
        for i in 0..m {
            for j in 0..n {
                if map[i][j] == 0b11 {
                    ans.push(vec![i as i32, j as i32]);
                }
            }
        }

        ans
    }
}

fn search(map: &mut Vec<Vec<i32>>, heights: &Vec<Vec<i32>>, height:i32, i: usize, j: usize, mark: i32) {
    let m = heights.len();
    let n = heights[0].len();
    if i >= m || j >= n || map[i][j] & mark > 0 || heights[i][j] < height{
        return;
    }
    map[i][j] |= mark;
    // up
    search(map, heights, heights[i][j], i - 1, j, mark);
    // down
    search(map, heights, heights[i][j], i + 1, j, mark);
    // left
    search(map, heights, heights[i][j], i, j - 1, mark);
    // right
    search(map, heights, heights[i][j], i, j + 1, mark);
}
