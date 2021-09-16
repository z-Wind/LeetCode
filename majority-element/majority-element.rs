// Boyer-Moore Voting Algorithm
// 刪去一個數列中的兩個不同的數字，不會影響該數列的 majority element。
//
// 假想有一群人要投票，候選人有A、B、C，假設A已知會過半數的話，
// 任取其中2個人取消他們的投票資格，會有以下的狀況：
//
// 被取消資格的是B跟C -> 顯然A還是過半數(而且比例更高了XD)
// 被取消資格的是(A, B)或(A, C) -> 一個投A的和一個不投A的同步被排除，
// 所以無法改變A會過半數的狀況。
// n > len/2  =>  n-1 > (len-2)/2 = len/2 - 1
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut candidate = nums[0];

        for num in nums {
            if (count == 0) {
                candidate = num;
            }
            match num == candidate{
                true => count+=1,
                false => count-=1,
            }
        }

        candidate
    }
}