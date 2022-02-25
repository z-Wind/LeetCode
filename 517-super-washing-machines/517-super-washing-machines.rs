// https://leetcode.com/problems/super-washing-machines/discuss/99177/Very-intuitive-O(n)-solution
// https://leetcode.com/problems/super-washing-machines/discuss/99181/C++-16ms-O(n)-solution-(with-trivial-proof)/103242

impl Solution {
    pub fn find_min_moves(machines: Vec<i32>) -> i32 {
        let n = machines.len();
        let sum: i32 = machines.iter().sum();
        if sum % n as i32 != 0 {
            return -1;
        }

        let avg = sum / n as i32;
        
        let mut leftSums = vec![0; n];
        for i in 1..n {
            leftSums[i] = leftSums[i - 1] + machines[i - 1];
        }
        // println!("{:?}", leftSums);
        
        let mut rightSums = vec![0; n];
        for i in (0..n - 1).rev() {
            rightSums[i] = rightSums[i + 1] + machines[i + 1];
        }
        // println!("{:?}", rightSums);
        
        let mut mov = 0;
        for i in 0..n {
            let expLeft = i as i32 * avg;
            let left = 0.max(expLeft - leftSums[i]);
            
            let expRight = (n - i - 1) as i32 * avg;
            let right = 0.max(expRight - rightSums[i]);
            
            mov = mov.max(left + right);
        }
        
        mov
    }
}
