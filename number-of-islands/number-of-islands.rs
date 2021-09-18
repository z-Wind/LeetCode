impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let mut count = 0;
        let m = grid.len();
        let n = grid[0].len();
        for i in (0..grid.len()){
            for j in (0..grid[0].len()){
                if grid[i][j] == '1'{
                    island_connect(&mut grid, m, n, i, j);
                    count += 1;
                }
            }
        }
        // println!("{:?}", grid);
        count
    }
}

fn island_connect(grid: &mut Vec<Vec<char>>, m:usize, n:usize, i:usize, j:usize){
    if i >= m || j >= n || grid[i][j] != '1'{
        return;
    }
    
    grid[i][j] = '_';
    island_connect(grid,m,n,i-1,j);
    island_connect(grid,m,n,i+1,j);
    island_connect(grid,m,n,i,j-1);
    island_connect(grid,m,n,i,j+1);

}