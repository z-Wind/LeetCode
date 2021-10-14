// https://github.com/z-Wind/LeetCode/blob/main/ugly-number-ii/ugly-number-ii.rs
// nums[1] = min( nums[0]x2, nums[0]x3, nums[0]x5)
// nums[2] = min( nums[1]x2, nums[0]x3, nums[0]x5)
// And so on. Be careful about the cases such as 6, in which we need to forward both pointers of 2 and 3.

impl Solution {
    pub fn nth_super_ugly_number(n: i32, primes: Vec<i32>) -> i32 {
        if n == 1{
            return 1;    
        }
        let n = n as usize;
        let mut nums = vec![1];
        
        let mut idx:Vec<usize> = vec![0;primes.len()];
        while nums.len() < n{
            let v:Vec<(usize,i32)> = idx.iter()
                          .enumerate()
                          .map(|(i,&x)| (i,nums[x]*primes[i]))
                          .collect();
            let &(_,next) = v.iter().min_by(|(_,a),(_,b)| a.cmp(b)).unwrap();
            nums.push(next);
            for i in 0..v.len(){
                if v[i].1 == next{
                    idx[i] += 1;      
                }
            }
        }
        // println!("{:?}", nums);
        nums[n-1]
    }
}