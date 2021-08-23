// https://abhinandandubey.github.io/posts/2019/12/15/Largest-Rectangle-In-Histogram.html
impl Solution {
    pub fn largest_rectangle_area(mut heights: Vec<i32>) -> i32 {
        if (heights.len() == 0) {
            return 0;
        }
        let mut maxArea = 0;
        let mut stack:Vec<(i32,i32)> = vec![(-1,0)];
        heights.push(0);
        for (i,h) in heights.into_iter().enumerate(){
            //println!("{:?}", stack);
            while h < stack.last().unwrap().1{
                let (_,h_stack) = stack.pop().unwrap();
                let w = i as i32 - stack.last().unwrap().0 -1;
                maxArea = maxArea.max(w*h_stack);
            } 
            stack.push((i as i32, h));
        }
    
        return maxArea;
    }
}