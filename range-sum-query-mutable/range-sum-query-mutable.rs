// https://medium.com/@edison.cy.yang/explaining-the-binary-indexed-tree-34f27ad0a513
// https://zh.wikipedia.org/wiki/%E6%A0%91%E7%8A%B6%E6%95%B0%E7%BB%84
// https://cp.wiwiho.me/fenwick-tree/
// https://yuihuang.com/binary-indexed-tree/?utm_source=rss&utm_medium=rss&utm_campaign=binary-indexed-tree

#[derive(Debug)]
struct BinaryIndexedTree {
    vals:Vec<i32>,
    len:usize,
}

impl BinaryIndexedTree {
    // O(n)
    fn new(nums: &[i32]) -> Self {
        let mut tree = Self{
            vals:vec![0;nums.len()+1],
            len:nums.len(),
        };
        for i in (0..nums.len()){
            tree.update(i, nums[i]);
        }
        // println!("{:?}", tree.vals);
        tree
    }
    
    fn low_bit(&self, x:usize) -> usize{
        // get last bit equal 1
        //             3       6
        //  x        011    0110
        // -x        101    1010
        // x & (-x)  001    0010
        let x = x as i32;
        (x & (-x)) as usize
    }

    // O(logn)
    fn update(&mut self, i: usize, val: i32) {        
        // get previous value
        let prev: i32 = self.query(i) - self.query(i - 1);
        let delta: i32 = val - prev;
        
        let mut i = i + 1;
        
        // update prefix sum in BIT
        while(i <= self.len){
            self.vals[i as usize] += delta;
            i += self.low_bit(i);
        }
        // println!("{:?}", self.vals);
    }

    // O(logn)
    // nums[0..=i].sum
    fn query(&self, i: usize) -> i32 {
        let mut i = i+1;
        let mut sum = 0;
        while i > 0{
            sum += self.vals[i];
            i -= self.low_bit(i);
        }
        sum
    }
}

#[derive(Debug)]
struct NumArray {
    bit:BinaryIndexedTree,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {

    fn new(nums: Vec<i32>) -> Self {
        let bit = BinaryIndexedTree::new(&nums);
        Self{
            bit,
        }
    }
    
    fn update(&mut self, index: i32, val: i32) {
        let index = index as usize;
        self.bit.update(index, val);
    }
    
    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.bit.query(right as usize) - self.bit.query(left as usize - 1)
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * obj.update(index, val);
 * let ret_2: i32 = obj.sum_range(left, right);
 */