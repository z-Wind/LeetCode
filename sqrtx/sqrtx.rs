impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x <= 1{
            return x;
        }
        let mut val = 1;
        let mut shift = 0;
        while val < (x>>2){
            val <<= 2;
            shift += 2;
            //println!("shift:{}, val:{}",shift, val);
        }
        let mut root = (1 << shift/2);
        //println!("root:{}, val:{}",root, val);
        let mut square = root * root;
        while square <= x{
            root += 1;
            square = match root.checked_mul(root){
                Some(x) => x,
                None => break,
            };
            //println!("root:{}, square:{}",root, square);
        }
        
        return root-1;
    }
}