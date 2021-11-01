use std::ops::RangeInclusive;
impl Solution {
    pub fn is_self_crossing(distance: Vec<i32>) -> bool {
        let n = distance.len();
        if n < 4{
            return false;
        }
        let mut up:Option<(RangeInclusive<i32>, i32)> = None;
        let mut temp_up:Option<(RangeInclusive<i32>, i32)> = None;
        let mut down:Option<(RangeInclusive<i32>, i32)> = None;
        let mut temp_down:Option<(RangeInclusive<i32>, i32)> = Some(((0..=0),0));
        let mut left:Option<(i32, RangeInclusive<i32>)> = None;
        let mut temp_left:Option<(i32, RangeInclusive<i32>)> = None;
        let mut right:Option<(i32, RangeInclusive<i32>)> = None;
        let mut temp_right:Option<(i32, RangeInclusive<i32>)> = None;
        let mut x = 0;
        let mut y = 0;
        for i in 0..n{
            // println!("{}:{}, {},{}",i,distance[i],x,y);
            // println!("up   :{:?} => {:?}",up, temp_up);
            // println!("down :{:?} => {:?}",down, temp_down);
            // println!("left :{:?} => {:?}",left, temp_left);
            // println!("right:{:?} => {:?}",right, temp_right);
            // println!("");
            match i%4{
                0 => {
                    let move_y = y + distance[i];
                    if let Some((x_r,y_l)) = &down{
                        if &y <= y_l && &move_y >= y_l && x_r.contains(&x){
                            return true;
                        }
                    }
                    if let Some((x_r,y_l)) = &up{
                        if &move_y >= y_l && x_r.contains(&x){
                            return true;
                        }
                    }
                    down = temp_down.clone();
                    temp_right = Some((x, y..=move_y));
                    y = move_y;
                },
                2 => {
                    let move_y = y - distance[i];
                    if let Some((x_r,y_l)) = &down{
                        if &move_y <= y_l && x_r.contains(&x){
                            return true;
                        }
                    }
                    if let Some((x_r,y_l)) = &up{
                        if &y >= y_l && &move_y <= y_l && x_r.contains(&x){
                            return true;
                        }
                    }
                    up = temp_up.clone();
                    temp_left = Some((x, move_y..=y));
                    y = move_y;
                },
                3 => {
                    let move_x = x + distance[i];
                    if let Some((x_l,y_r)) = &left{
                        if &x <= x_l && &move_x >= x_l && y_r.contains(&y){
                            return true;
                        }
                    }
                    if let Some((x_l,y_r)) = &right{
                        if &move_x >= x_l && y_r.contains(&y){
                            return true;
                        }
                    }
                    left = temp_left.clone();
                    temp_down = Some((x..=move_x, y));
                    x = move_x;
                },
                1 => {
                    let move_x = x - distance[i];
                    if let Some((x_l,y_r)) = &left{
                        if &move_x <= x_l && y_r.contains(&y){
                            return true;
                        }
                    }
                    if let Some((x_l,y_r)) = &right{
                        if &x >= x_l && &move_x <= x_l && y_r.contains(&y){
                            return true;
                        }
                    }
                    right = temp_right.clone();
                    temp_up = Some((move_x..=x, y));
                    x = move_x;
                },
                _ => panic!("impossible"),
            }
        }
        false
    }
}