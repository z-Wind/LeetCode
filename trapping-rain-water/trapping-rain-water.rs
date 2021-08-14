use std::cmp::{min,max};

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut waters=0;
        let mut i=0;
        let mut set:Vec<(usize,usize,i32)> = Vec::new();
        while i < height.len(){
            let (start, end) = find_trap(i, &height);
            
            if start == i || end == i{
                i+=1;    
                continue
            } 
            
            let mut water = cal_water(start, end, &height);
            let mut push = true;
            let set_len = set.len();

            for ii in (0..set_len).rev(){
                let &(m,n,w) = &set[ii];
                if m <= start && n>=end{
                    water = 0;
                    push = false;
                } else if start <= m && end >= n{
                    waters -= w;
                    set.remove(ii);
                }
            }
            if push{
                set.push((start,end,water));
            }
            waters += water;
            //println!("{} => {} to {} : {} get:{}",i, start, end, water, waters);
            i=end+1;
        }
        waters
    }
}

fn cal_water(start:usize, end:usize, height:&Vec<i32>) -> i32{
    let mut water = 0;
    let min_h = min(height[start], height[end]);
    for i in (start+1..end){
        water += (min_h - height[i]);
    }
    
    water
}

fn find_trap(mid:usize, height:&Vec<i32>) -> (usize, usize){
    let mut start = mid;
    let mut end = mid;
    
     
    let mut i = mid;
    let mut j = mid;
    let mut max_left = height[mid];
    let mut max_right = height[mid];
    while i as i32 >=0 && j < height.len(){
        //println!("mid:{} => {}:{}, {}:{} => {}, {}",mid,i,max_left,j,max_right,start,end);
        if i as i32 >=0 && height[i] > max_left{
            max_left = height[i];
            start = i;
        }
        if j < height.len() && height[j] > max_right{
            max_right = height[j];
            end = j;
        }
        
        if j < height.len() && max_right >= max_left{
            i-=1;
        }
        if i as i32 >=0 && max_left >= max_right{
            j+=1;    
        }
    }
    
    //println!("get: {},{}",start, end);
    (start, end)
}