use std::cmp::max;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut i = 0;
        let mut maxReachable = 0;
        let mut lastJumpedPos = 0;
        let mut jumps = 0;
	    // loop till last jump hasn't taken us till the end
        while(lastJumpedPos < n - 1) {  
            // furthest index reachable on the next level from current level
		    maxReachable = max(maxReachable, i + nums[i] as usize);  
            // current level has been iterated & maxReachable position on next level has been finalised
            if(i == lastJumpedPos) {			  
                // so just move to that maxReachable position
                lastJumpedPos = maxReachable;     
                // and increment the level
                jumps+=1;                          
        // NOTE: jump^ only gets updated after we iterate all possible jumps from previous level
        //       This ensures jumps will only store minimum jump required to reach lastJumpedPos
		}            
		i+=1;
	}
	return jumps;
    }
}