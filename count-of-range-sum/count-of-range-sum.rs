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
        let mut temp = vec![0;n+1];
        for i in 1..=n {
            sum[i] = sum[i-1] + nums[i-1] as i64;
        }
        
        // println!("orig : {:?}", sum);
        self.mergesort(&mut sum , 0, n, &mut temp);
        // println!("final: {:?}", sum);
        self.count
    }
    
    fn mergesort(&mut self, sum:&mut Vec<i64>, start:usize, end:usize, temp:&mut Vec<i64>) {
        // println!("merge: {:?}", &sum[start..=end]);
        if start >= end {
            return;
        }
        let mid = start + (end - start) / 2;
        self.mergesort(sum, start, mid, temp);
        self.mergesort(sum, mid + 1, end, temp);
        self.merge(sum, start, mid, end, temp);
    }
    
    fn merge(&mut self,sum:&mut Vec<i64>, start:usize, mid:usize, end:usize, temp:&mut Vec<i64>) {
        let mut right = mid + 1;
        let mut index = start;
        let mut low = mid + 1;
        let mut high = mid + 1;
        for left in start..=mid{
            while right <= end && sum[right] < sum[left] {
                temp[index] = sum[right];
                index += 1;
                right += 1;
            }
            temp[index] = sum[left];
            index += 1;
            
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
            temp[index] = sum[right];
            index += 1;
            right += 1;
        }
        
        for i in start..=end {
            sum[i] = temp[i];
        }
        // println!("{}: merged: {:?}", self.count, &sum[start..=end]);
    }
}