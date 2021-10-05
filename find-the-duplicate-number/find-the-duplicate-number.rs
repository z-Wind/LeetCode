// Floyd's algorithm
// F: the number of nodes to circle
// a: the intersection which from start of circule
// rabbit is twice as fast as tortoise
//
// Phase 1: from start
// tortoise walk F + a distance
// rabbit walk F + nC + a distance
// 2(F+a) = F + nC + a
// F+a = nC
//
// Phase 2: tortoise from start, rabbit move one step from intersection
// because F+a = nC
// the intersection would be the start of circule

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        // phase 1
        let mut tortoise = nums[0];
        let mut rabbit = nums[0];
        loop{
            tortoise = nums[tortoise as usize];
            rabbit = nums[nums[rabbit as usize] as usize];
            if tortoise == rabbit{
                break;
            }
        }
        // println!("from start, F+a:{}, F+nC+a:{}", tortoise, rabbit);
        
        // phase 2
        tortoise = nums[0];
        while tortoise != rabbit{
            tortoise = nums[tortoise as usize];
            rabbit = nums[rabbit as usize];
        }
        
        tortoise
    }
}