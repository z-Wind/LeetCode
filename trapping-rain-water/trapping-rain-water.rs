use std::cmp::{min,max};

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        trap(&height[..])
    }
}

fn cal_water(start:usize, end:usize, height:&[i32]) -> i32{
    let mut water = 0;
    let min_h = min(height[start], height[end]);
    for i in (start+1..end){
        water += (min_h - height[i]);
    }
    
    water
}

fn trap(height: &[i32]) -> i32 {
    //println!("{:?}",height);
    if height.len() < 3{
        return 0;
    }
    
    let mut waters=0;
    let (start, end) = max_area(&height);
    //println!("{},{}",start,end);
    if end-start < 2{
        waters += trap(&height[..start+1]) + trap(&height[end..]);
    } else {
        let mut water = cal_water(start, end, &height);

        //println!("{},{} => {}",start,end,water);
        waters += water + trap(&height[..start+1]) + trap(&height[end..]);
    }
    
    waters
}

fn max_area(height: &[i32]) -> (usize,usize) {
    let mut l:usize = 0;
    let mut r:usize = height.len() - 1;
    let mut max_left:usize = l;
    let mut max_right:usize = r;
    while (l < r) {
        if (height[max_left] < height[l]) {
            max_left = l;
        }
        if (height[max_right] < height[r]) {
            max_right = r;
        }
        
        if (height[l] < height[r]) {
            l+=1;
        }
        else{
            r-=1;
        }
    }
    return (max_left,max_right);
}