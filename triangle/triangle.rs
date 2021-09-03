// minpath[row][col] = triangle[row][col] + min( minpath[row+1][col], minpath[row+1][col+1] );
// bottom to top
impl Solution {
    pub fn minimum_total(mut triangle: Vec<Vec<i32>>) -> i32 {
        for row in (0..triangle.len()-1).rev(){
            for col in (0..triangle[row].len()){
                triangle[row][col] += triangle[row+1][col].min(triangle[row+1][col+1])
            }
        }
        triangle[0][0]
    }
}