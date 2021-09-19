impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        let n = n as usize;
        let mut is_prime = vec![false;n];
        let mut count = 0;
        for i in (2..n) {
            if !is_prime[i] {
                count+=1;
                for j in (2..).take_while(|j| i*j<n) {
                    is_prime[i*j] = true;
                }
            }
        }
        
        count
    }
}