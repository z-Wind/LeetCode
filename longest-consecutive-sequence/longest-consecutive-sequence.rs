impl Solution {
    pub fn longest_consecutive(mut nums: Vec<i32>) -> i32 {
        if nums.is_empty(){
            return 0;
        }
        nums.sort();
        // println!("{:?}", nums);
        let mut max_count = 0;
        let mut count = 1;
        for w in nums.windows(2){
            if (w[0] + 1) == w[1]{
                count += 1;
            } else if w[0] == w[1]{
                continue;
            } else {
                max_count = max_count.max(count);
                count = 1;
            }
        }
        max_count = max_count.max(count);
        max_count
    }
}