use std::collections::HashSet;
impl Solution {
    pub fn is_rectangle_cover(rectangles: Vec<Vec<i32>>) -> bool {
        let mut set_bottom_left:HashSet<(i32,i32)> = HashSet::new();
        let mut set_bottom_right:HashSet<(i32,i32)> = HashSet::new();
        let mut set_top_left:HashSet<(i32,i32)> = HashSet::new();
        let mut set_top_right:HashSet<(i32,i32)> = HashSet::new();
        for rec in rectangles{
            let bottom_left = (rec[0],rec[1]);
            if set_bottom_left.contains(&bottom_left){
                return false;
            }
            let bottom_right = (rec[2],rec[1]);
            if set_bottom_right.contains(&bottom_right){
                return false;
            }
            let top_left = (rec[0],rec[3]);
            if set_top_left.contains(&top_left){
                return false;
            }
            let top_right = (rec[2],rec[3]);
            if set_top_right.contains(&top_right){
                return false;
            }
            
            combine(bottom_left, &mut set_bottom_left, &mut set_bottom_right, &mut set_top_left);
            combine(bottom_right, &mut set_bottom_right, &mut set_bottom_left, &mut set_top_right);
            combine(top_left, &mut set_top_left, &mut set_bottom_left, &mut set_top_right);
            combine(top_right, &mut set_top_right, &mut set_bottom_right, &mut set_top_left);
            
            // println!("bottom_left:{:?}", bottom_left);
            // println!("bottom_right:{:?}", bottom_right);
            // println!("top_left:{:?}", top_left);
            // println!("top_right:{:?}", top_right);
            // println!("set_bottom_left:{:?}", set_bottom_left);
            // println!("set_bottom_right:{:?}", set_bottom_right);
            // println!("set_top_left:{:?}", set_top_left);
            // println!("set_top_right:{:?}", set_top_right);
            // println!("");
        }
        
        set_bottom_left.len() == 1 && set_bottom_right.len() == 1 && set_top_left.len() == 1 && set_top_right.len() == 1
    }
}

fn combine(xy:(i32,i32), set:&mut HashSet<(i32,i32)>, set1:&mut HashSet<(i32,i32)>, set2:&mut HashSet<(i32,i32)>){
    if set1.contains(&xy){
        set1.remove(&xy);
    } else if set2.contains(&xy){
        set2.remove(&xy);
    } else {
        set.insert(xy);
    }
}