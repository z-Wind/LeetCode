// https://github.com/z-Wind/LeetCode/blob/main/largest-rectangle-in-histogram/largest-rectangle-in-histogram.rs
impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut v:Vec<i32> = vec![0;n];
        let mut max_square = 0;
        for i in (0..m){
            for j in (0..n){
                match matrix[i][j]{
                    '0' => v[j] = 0,
                    '1' => v[j] += 1,
                    _ => panic!(),
                }
            }
            let area = largest_square_area(&mut v);
            // println!("{}: {:?}",area, v);
            max_square = max_square.max(area);
        }
        max_square
    }
}

fn largest_square_area(heights: &mut Vec<i32>) -> i32 {
    if (heights.len() == 0) {
        return 0;
    }
    let mut maxArea = 0;
    let mut stack:Vec<(i32,i32)> = vec![(-1,0)];
    heights.push(0);
    for (i,&h) in heights.iter().enumerate(){
        // println!("{:?}", stack);
        while h < stack.last().unwrap().1{
            let (_,h_stack) = stack.pop().unwrap();
            let w = i as i32 - stack.last().unwrap().0 -1;
            let square_side = w.min(h_stack);
            maxArea = maxArea.max(square_side*square_side);
        } 
        stack.push((i as i32, h));
    }
    heights.pop();

    return maxArea;
}