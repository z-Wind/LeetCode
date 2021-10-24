// https://leetcode.com/problems/count-of-range-sum/discuss/77990/Share-my-solution
//
// Recall count smaller number after self where we encountered the problem
// count[i] = count of nums[j] - nums[i] < 0 with j > i
// Here, after we did the preprocess, we need to solve the problem
// count[i] = count of a <= S[j] - S[i] <= b with j > i
// ans = sum(count[:])
//
// like https://github.com/z-Wind/LeetCode/blob/main/count-of-smaller-numbers-after-self/count-of-smaller-numbers-after-self.rs

impl Solution {
    pub fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let mut c = CountRangeSum::new(lower as i64, upper as i64);
        c.count(nums)
    }
}

struct CountRangeSum{
    count:i32,
    lower:i64,
    upper:i64,
}

impl CountRangeSum{
    fn new(lower: i64, upper: i64) -> Self{        
        Self{
            count:0,
            lower,
            upper,
        }
    }
    fn count(&mut self, nums: Vec<i32>) -> i32 {
        let n = nums.len();
        // n+1 for only one nums, like nums[0..=0]
        let mut sum = vec![0;n+1];
        sum[0] = 0;
        for i in 1..=n {
            sum[i] = sum[i-1] + nums[i-1] as i64;
        }
        
        // println!("orig : {:?}", sum);
        self.mergesort(&mut sum , 0, n);
        // println!("final: {:?}", sum);
        self.count
    }
    
    fn mergesort(&mut self, sum:&mut Vec<i64>, start:usize, end:usize) {
        // println!("merge: {:?}", &sum[start..=end]);
        if start >= end {
            return;
        }
        let mid = start + (end - start) / 2;
        self.mergesort(sum, start, mid);
        self.mergesort(sum, mid + 1, end);
        self.merge(sum, start, mid, end);
    }
    
    fn merge(&mut self,sum:&mut Vec<i64>, start:usize, mid:usize, end:usize) {
        let mut right = mid + 1;
        let mut low = mid + 1;
        let mut high = mid + 1;
        let mut merged:Vec<i64> = Vec::with_capacity(end-start+1);
        
        for left in start..=mid{
            while right <= end && sum[right] < sum[left] {
                merged.push(sum[right]);
                right += 1;
            }
            merged.push(sum[left]);
            
            // extra action for count
            while low <= end && sum[low] - sum[left] < self.lower {
                low+=1;
            }
            while high <= end && sum[high] - sum[left] <= self.upper {
                high+=1;
            }
            self.count += (high - low) as i32;
        }
        while right <= end {
            merged.push(sum[right]);
            right += 1;
        }
        
        // part of normal merge sort
        // copy back merged result into array
        let mut pos = start;
        for num in merged {
            sum[pos] = num;
            pos+=1;
        }
        // println!("{}: merged: {:?}", self.count, &sum[start..=end]);
    }
}