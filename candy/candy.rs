#[derive(Debug)]
enum Order {
    ASC,
    DESC,
    EQ,
    UNKNOW,
}
use std::cmp::min;
impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let mut left:usize = 0;
        let mut right:usize = 0;
        let mut mid:usize = 0;
        let mut sum:i32 = 0;
        
        let mut state = Order::UNKNOW;
        let mut pre_state = Order::UNKNOW;
        for (i,w) in ratings.windows(2).enumerate(){
            right = i;
            // println!("{:?} {},{},{}",w,left,mid,right);
            if w[1] == w[0]{
                state = Order::EQ;
            } else if w[1] < w[0]{
                state = Order::DESC;
            } else if w[1] > w[0]{
                state = Order::ASC;
            } else {
                state = Order::UNKNOW;
            }
                    
            match (&pre_state, &state){
                (Order::UNKNOW, Order::ASC) | 
                (Order::ASC, Order::ASC) | 
                (Order::UNKNOW, Order::DESC) |
                (Order::DESC, Order::DESC) |
                (Order::EQ, Order::ASC) | 
                (Order::EQ, Order::DESC) => (),
                // 1 2 3 2 1 
                (Order::ASC,Order::DESC) => {
                    mid = i;
                },
                // 1 2 2
                // 2 2 1
                // 1 3 2 2
                (Order::ASC,Order::EQ) | 
                (Order::UNKNOW,Order::EQ) |
                (Order::DESC,Order::EQ) => {
                    sum += candy_sum(left,mid,right);
                    left = right + 1;
                    mid = right + 1;  
                },
                // 1 2 3 2 1 2
                (Order::DESC,Order::ASC) => {
                    sum += candy_sum(left,mid,right);
                    sum -= 1;
                    left = right;
                    mid = right; 
                },
                // 2 2 2
                (Order::EQ,Order::EQ) => {
                    sum += 1;  
                    left = right + 1;
                    mid = right + 1; 
                },
                (pre,cur) => panic!(format!("{:?} {},{},{} => {:?},{:?}",ratings,left,mid,right,pre,cur)),
            }
            
            pre_state = state;
        }
        // for 1 2 3 or 3 2 1
        sum += candy_sum(left,mid,ratings.len()-1);
        
        sum
    }
}

// for 2 3 4 3 2 1 or 1 2 or 4 3
fn candy_sum(left:usize, mid:usize, right:usize) -> i32 {
    let left_n = (mid - left + 1) as i32;
    let right_n = (right - mid + 1) as i32;
    let mut sum = 0;
    
    if left_n >= right_n{
        sum = (left_n+1)*left_n/2 + (right_n-1)*right_n/2;
    } else {
        sum = (right_n+1)*right_n/2 + (left_n-1)*left_n/2;
    }
    
    // println!("{},{},{} => {}",left,mid,right,sum);
    sum
}
