// Lexicographic (Binary Sorted) Subsets
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];
        
        for i in 0..2_u32.pow(nums.len() as u32) {
            let mut subset = vec![];
            let binary_str = format!("{:0n$b}", i,n=nums.len());
            //println!("{}",binary_str);
            for (bit_index, bit) in binary_str.chars().enumerate() {
                if bit == '1' {
                    subset.push(nums[bit_index]);
                }
            }
            res.push(subset);
        }
        
        res
    }
}