impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        if n <= 2 {
            return 0;
        }
        let n = n as usize;
        
        let mut is_prime = vec![true;n];
        is_prime[0] = false;
        is_prime[1] = false;
        
        let sqrted_n = (n as f64).sqrt() as usize;
        for i in (2..=sqrted_n) {
            if is_prime[i] {
                let mut j = i*i;
                while j<n {
                    is_prime[j] = false;
                    j+=i;
                }
            }
        }
        
        is_prime.iter().filter(|x| **x).count() as i32
    }
}