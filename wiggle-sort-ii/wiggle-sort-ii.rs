// https://leetcode.com/problems/wiggle-sort-ii/discuss/77678/3-lines-Python-with-Explanation-Proof

impl Solution {
    pub fn wiggle_sort(nums: &mut Vec<i32>) {
        let mut temp = nums.clone();
        temp.sort();
        let n = temp.len();
        let mid = (n+1)/2;
        // println!("{} {} {:?}", n,n/2,temp);
        for i in (0..temp.len()-1).step_by(2){
            // println!("{},{} => {},{}",i,i+1,temp[mid-1-i/2],temp[n-1-i/2]);
            nums[i] = temp[mid-1-i/2];
            nums[i+1] = temp[n-1-i/2];
        }
        if n%2 == 1{
            nums[n-1] = temp[0];
        }
    }
}
