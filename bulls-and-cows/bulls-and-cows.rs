impl Solution {
    pub fn get_hint(secret: String, guess: String) -> String {
        let mut digits = vec![0;10];
        let mut bulls = 0;
        let mut cows = 0;
        let secret = secret.as_bytes();
        let guess = guess.as_bytes();
        for i in (0..secret.len()){
            if secret[i] == guess[i]{
                bulls += 1;
                continue;
            }
            
            let mut idx = (secret[i] - b'0') as usize;
            if digits[idx] < 0{
                cows+=1;
            }
            digits[idx] += 1;
            idx = (guess[i] - b'0') as usize;
            if digits[idx] > 0{
                cows+=1;
            }
            digits[idx] -= 1;
        }
        
        format!("{}A{}B",bulls,cows)
    }
}