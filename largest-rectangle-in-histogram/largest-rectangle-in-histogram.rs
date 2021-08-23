use std::cmp::max;
impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        find(&heights[..])
    }
}

fn find(heights: &[i32]) -> i32 {
    if heights.len() == 0{
        return 0;
    }
    let mut min_h:i32 = i32::MAX;
    let mut min_i = 0;
    let mut count = 0;
    for (i,h) in heights.iter().enumerate(){
        if h < &min_h{
            min_h = *h;
            min_i = i;
            count = 1;
        } else if h == &min_h{
            count += 1;
        }
    }
    let area = min_h * heights.len() as i32;
    if count == heights.len(){
        return area;
    }
    let split_area = max(find(&heights[..min_i]), find(&heights[min_i+1..]));
    max(area, split_area)
}