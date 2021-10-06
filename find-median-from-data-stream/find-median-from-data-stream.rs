use std::collections::BTreeMap;
struct MedianFinder {
    nums:BTreeMap<i32,usize>,
    len:usize,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {

    fn new() -> Self {
        Self{
            nums:BTreeMap::new(),
            len:0,
        }
    }
    
    fn add_num(&mut self, num: i32) {
        *self.nums.entry(num).or_insert(0)+=1;
        self.len += 1; 
        // println!("{:?}:{}", self.nums,self.len);
    }
    
    fn find_median(&self) -> f64 {
        let (x,y) = if self.len%2 == 0{
            let i = self.len/2;
            (i,i+1)
        } else {
            let i = self.len/2;
            (i+1,i+1)
        };
        // println!("{},{}",x,y);
        let mut count = 0;
        let mut a = None;
        let mut b = None;
        for (key, value) in self.nums.iter() {
            count += value;
            if count >= x && a.is_none(){
                a = Some(key);
            }
            if count >= y && b.is_none(){
                b = Some(key);
                break;
            }
        }
        // println!("{:?},{:?}",a,b);
        (a.unwrap() + b.unwrap()) as f64 / 2.0
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */