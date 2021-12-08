// https://leetcode.com/problems/split-array-largest-sum/discuss/89819/C%2B%2B-Fast-Very-clear-explanation-Clean-Code-Solution-with-Greedy-Algorithm-and-Binary-Search

impl Solution {
    pub fn split_array(nums: Vec<i32>, m: i32) -> i32 {
        // Use i64 to avoid overflow.
        let mut left: i64 = 0;
        let mut right: i64 = 0;
        // The smallest possible value ('left') is the the value of the largest element in this array.
        // The largest possible value ('right') is the sum of all elements in this array.
        for &num in nums.iter() {
            let num = num as i64;
            left = left.max(num);
            right += num;
        }

        // Use binary search, find the lower bound of the possible (minimum sum of groups within m - 1 cuts).
        while left < right {
            let mid = left + (right - left) / 2;
            if doable(&nums, m - 1, mid) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        return left as i32;
    }
}

fn doable(nums: &Vec<i32>, mut cuts: i32, max: i64) -> bool {
    // 'acc' is the temporary accumulator for the currently processed group.
    let mut acc = 0;
    for &num in nums.iter() {
        let num = num as i64;
        // If the current processed element in this array is larger than 'max', we cannot segment the array.
        // (Reason is straightforward, if 'nums' is [10, 2, 3, 5] and 'max' is 6, even you can have 3 cuts
        // (by which you can cut array as [[10], [2], [3], [5]]), the group containing 10 will be larger than 6,
        //  there is no way to do this).
        // Ps: This step is unnecessary in this solution. Because 'left' in the splitArray() function can assure
        // 'max' will be larger than every single element. I just want to write a generalized doable() function :)

        if num > max {
            return false;
        }
        // If the (sum of the currently processed group) + (current element) is smaller than max, we can add current
        // element into this group.
        else if acc + num <= max {
            acc += num;
        }
        // If not, we will make a cut before this element, and this element will be the first element in the new group.
        else {
            cuts -= 1;
            // If we've used up all cuts, this means this 'max' is not doable.
            if (cuts < 0) {
                return false;
            }

            acc = num;
        }
    }

    // If we can reach here, this means we've used at most 'cuts' cut to segment the array, and the sum of each groups is
    // not larger than 'max'. Yeah!
    return true;
}
