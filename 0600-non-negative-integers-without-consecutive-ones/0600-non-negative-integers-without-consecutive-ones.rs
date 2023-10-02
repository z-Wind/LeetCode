//  https://leetcode.com/problems/non-negative-integers-without-consecutive-ones/solutions/103754/c-non-dp-o-32-fibonacci-solution/

// 由左至右，每逢 1 就分成兩部分
// 該 bit 設為 0 可替換為 f(k)
// 該 bit 設為 1 則為新起點，若前一個 bit 為 1，則中斷，因具有連續兩個 1
// f(k) = f(k-1) + f(k-2), where k>=2, f(0) = 1, f(1) =2;
// f(k) means the count    without Consecutive Ones, from 0, to 2^k -1;
// k is the length of binary string , k = len(bin(2^k -1)) for example :
// f(5) = count(00000 - 11111),
// count(00000 - 11111) = count(00000 - 01111) + count(10000 - 11111)
//                      = f(4) + count(10000 - 10111) + count(11000 - 11111)
//                      = f(4) + f(3)
//
// count(00000000 - 10010110) = count(00000000 - 01111111) + count(10000000 - 10010110)
//                            = f(7) + count(10000000 - 10001111) + count(10010000 - 10010110)
//                            = f(7) + f(4) + count(10010000 - 10010011) + count(10010100 - 10010110)
//                            = f(7) + f(4) + f(2) + count(10010100 - 10010101) + count(10010110 - 10010110)
//                            = f(7) + f(4) + f(2) + f(1)

impl Solution {
    pub fn find_integers(n: i32) -> i32 {
        let mut f = [0; 32];
        f[0] = 1;
        f[1] = 2;
        // 依定義 f(k) = f(k-1) + f(k-2) 得到所有值
        for i in 2..32 {
            f[i] = f[i - 1] + f[i - 2];
        }

        let mut ans = 0;
        let mut k = 31;
        let mut pre_bit = 0;
        while k < 32 {
            if (n & (1 << k)) > 0 {
                ans += f[k];

                // 連兩個 1，直接回傳
                if pre_bit > 0 {
                    return ans;
                };

                pre_bit = 1;
            } else {
                pre_bit = 0;
            }
            k -= 1;
        }

        ans + 1 // 加本身
    }
}
