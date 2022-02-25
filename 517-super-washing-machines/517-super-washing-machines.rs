// https://leetcode.com/problems/super-washing-machines/discuss/99177/Very-intuitive-O(n)-solution

impl Solution {
    pub fn find_min_moves(machines: Vec<i32>) -> i32 {
        let n = machines.len();
        let sum: usize = machines.iter().sum::<i32>() as usize;
        if sum % n != 0 {
            return -1;
        }

        let avg = sum / n;
        let mut leftSums = vec![0; n];
        let mut rightSums = vec![0; n];
        for i in 1..n {
            leftSums[i] = leftSums[i - 1] + machines[i - 1] as usize;
        }
        for i in (0..n - 1).rev() {
            rightSums[i] = rightSums[i + 1] + machines[i + 1] as usize;
        }
        // println!("{:?}", leftSums);
        // println!("{:?}", rightSums);
        
        let mut mov = 0;
        for i in 0..n {
            let expLeft = i * avg;
            let expRight = (n - i - 1) * avg;
            let mut left = 0;
            let mut right = 0;
            if expLeft > leftSums[i] {
                left = expLeft - leftSums[i];
            } 
            if expRight > rightSums[i] {
                right = expRight - rightSums[i];
            }
            mov = mov.max(left + right);
        }
        mov as i32
    }
}
