// https://leetcode.com/problems/create-maximum-number/discuss/77285/Share-my-greedy-solution
// https://web.archive.org/web/20160120093629/http://algobox.org/create-maximum-number/
//
// select i nums from nums1 and k-i from nums2, and then merge the both.
// find the maximum candidate

impl Solution {
    pub fn max_number(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums1.len();
        let m = nums2.len();
        let k = k as usize;
        let mut ans = vec![];
        let mut start = if k - m > k { 0 } else { k - m };
        for i in start..=k.min(n) {
            let candidate = merge(&maxArray(&nums1, i), &maxArray(&nums2, k - i), k);
            if candidate > ans {
                ans = candidate;
            }
        }
        
        ans
    }
}

// nums1 = [6, 7]
// nums2 = [6, 0, 4]
// k = 5
// ans =[6, 7, 6, 0, 4]
fn merge(nums1: &Vec<i32>, nums2: &Vec<i32>, k: usize) -> Vec<i32> {
    // println!("\nmerge");
    // println!("nums1: {:?}", nums1);
    // println!("nums2: {:?}", nums2);
    let mut ans = vec![0; k];
    let mut i = 0;
    let mut j = 0;
    for r in 0..k {
        if nums1[i..] >=  nums2[j..] {
            ans[r] = nums1[i];
            i += 1;
        } else {
            ans[r] = nums2[j];
            j += 1;
        }
    }

    ans
}

fn maxArray(nums: &Vec<i32>, k: usize) -> Vec<i32> {
    let n = nums.len();
    let mut ans = vec![0; k];
    let mut j = 0;
    for i in 0..n {
        while n - i + j > k && j > 0 && ans[j - 1] < nums[i] {
            j -= 1;
        }
        if j < k {
            ans[j] = nums[i];
            j += 1;
        }
    }

    ans
}
