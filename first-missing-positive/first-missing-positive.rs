impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        for i in 0..n as usize {
            while 0 < nums[i] && nums[i] <= n && nums[i] != nums[nums[i] as usize -1] {
                let val = nums[i] as usize -1;
                nums.swap(i, val);
            }
        }

        //println!("{:?}",nums);
        for (i,&num) in nums.iter().enumerate() {
            let val = i as i32 + 1;
            if  val != num {
                return val;
            }
        }

        n + 1
    }
}