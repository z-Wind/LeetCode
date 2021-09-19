impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        if n <= 2{
            return 0;
        }
        
        let mut primes:Vec<i32> = vec![2];
        'outer: for i in (3..n).step_by(2){
            let root =  (i as f64).sqrt() as i32;
            for prime in primes.iter().take_while(|x| **x <= root){
                if i%prime == 0{
                    continue 'outer
                }
            }
            primes.push(i);
        }
        
        primes.len() as i32
    }
}