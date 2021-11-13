/** 
 * Forward declaration of guess API.
 * @param  num   your guess
 * @return 	     -1 if num is lower than the guess number
 *			      1 if num is higher than the guess number
 *               otherwise return 0
 * unsafe fn guess(num: i32) -> i32 {}
 */

impl Solution {
    unsafe fn guessNumber(n: i32) -> i32 {
        let mut left = 0;
        let mut right = n;
        let mut mid = 0;
        while left <= right{
            mid = left + (right-left)/2;
            match guess(mid){
                -1 => right = mid - 1,
                1 => left = mid + 1,
                _ => break,
            }
        }
        mid
    }
}