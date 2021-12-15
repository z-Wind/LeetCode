// https://leetcode.com/problems/maximum-xor-of-two-numbers-in-an-array/discuss/91059/Java-O(n)-solution-using-Trie
// https://leetcode.com/problems/maximum-xor-of-two-numbers-in-an-array/discuss/91059/Java-O(n)-solution-using-Trie/275175
// https://leetcode.com/problems/maximum-xor-of-two-numbers-in-an-array/discuss/849356/Rust-Trie-solution

// binary trie, where each trie node has only 2 child 
// one child will represent bit "0" in the binary representation of num
// one child will represent bit "1" in the binary representation of num
#[derive(Default,Debug)]
struct TrieNode{
    // children[0]: represent bit "0" in the binary representation 
    // children[1]: represent bit "1" in the binary representation 
    children: [Option<Box<TrieNode>>;2],
}
impl Solution {
    pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
        // initialize the root node
        let mut root:TrieNode = Default::default();
        
        // build the trie 
        for &num in nums.iter() {
            // starting from root;
            let mut curr = &mut root;
            
            // since every num is positive, their 31th bit is always 0
            // we can ignore that bit and directy build from the 30th bit 
            for i in (0..31).rev() {
                // get the ith bit (count from LSB, 0-based) of num
                let bit = ((num >> i) & 1) as usize;
                
                // if current bit is 0, it will go to children[0]
                // if current bit is 1, it will go to children[1]
                curr = curr.children[bit].get_or_insert_with(Default::default);
            }
        }
        
        let mut ans = i32::MIN;
        
        // iterate through each num again
        // starting from those significant bits of num, we try the best 
        // to go to the node that represent the negation of current bit 
        // if such node doesn't exist, we have to go the node that represents
        // the current bit 
        for &num in nums.iter() {
            // starting from root
            let mut curr = &root;
            
            // keep track of the maximum result of XOR current num with 
            // other num in the array
            let mut rst = 0;
            for i in (0..31).rev() {
                let bit = ((num >> i) & 1) as usize;
                
                // check to see if node that represents the negation of 
                // current bit exists or not. If exists, go to that way 
                
                // if current bit is 1, then we want to go to children[0] (which represents 0)
                // if current bit is 0, then we want to go to children[1] (which represents 1)
                if let Some(node) = &curr.children[1-bit] {
                    curr = node;
                    
                    // if exists, then we will have a "1" at the current index
                    // in the result of maximum XOR 
                    rst |= (1 << i);
                } 
                // if not exists
                else {
                    curr = &curr.children[bit].as_ref().unwrap();
                }
            }
            
            // keep track of global maximum 
            ans = ans.max(rst);
            // there is no need to continue when final result has reached max value 
            if ans == i32::MAX {
                break;  
            } 
        }
        
        return ans;
    }
}

