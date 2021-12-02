impl Solution {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        let k = k as usize;
        let n = num.len();
        if n == k{
            return String::from("0");
        }
        let ans = min_array(num.as_bytes(), n-k);
        let ans = String::from_utf8(ans).unwrap().trim_start_matches('0').to_string();
        if ans.len() == 0{
            return String::from("0");
        }
        ans
    }
}

fn min_array(nums: &[u8], k: usize) -> Vec<u8> {
    let n = nums.len();
    let mut ans = vec![0; k];
    let mut j = 0;
    for i in 0..n {
        while n - i + j > k && j > 0 && ans[j - 1] > nums[i] {
            j -= 1;
        }
        if j < k {
            ans[j] = nums[i];
            j += 1;
        }
    }

    ans
}