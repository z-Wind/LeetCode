// https://leetcode.com/problems/kth-smallest-element-in-a-sorted-matrix/discuss/1322101/C%2B%2BJavaPython-MaxHeap-MinHeap-Binary-Search-Picture-Explain-Clean-and-Concise

impl Solution {
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let k = k as usize;
        let (m, n) = (matrix.len(), matrix[0].len()); // For general, the matrix need not be a square

        let countLessOrEqual = |x| {
            let mut cnt = 0;
            let mut c = n - 1; // start with the rightmost column
            for r in 0..m {
                while c < n && matrix[r][c] > x {
                    c -= 1
                } // decrease column until matrix[r][c] <= x
                cnt += (c + 1);
            }
            return cnt;
        };

        let (mut left, mut right) = (matrix[0][0], matrix[m - 1][n - 1]);
        let mut ans = -1;
        while left <= right {
            let mid = (left + right) / 2;
            if countLessOrEqual(mid) >= k {
                ans = mid;
                right = mid - 1; // try to looking for a smaller value in the left side
            } else {
                left = mid + 1; // try to looking for a bigger value in the right side
            }
        }
        return ans;
    }
}
