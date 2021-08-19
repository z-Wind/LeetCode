impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        //println!("x:{}", x);
        if x == 0{
            return 0
        }
        let mut left = 1;
        let mut right = i32::MAX;
        loop{
            let mid = left + (right-left)/2;
            if x/mid >= mid && x/(mid+1) < mid+1{
                return mid;
            }
            else if x/mid > mid{
                left = mid+1;
            } else {
                right = mid-1;
            }
            //println!("mid:{}, left:{}, right:{}", mid, left, right);
        }
    }
}