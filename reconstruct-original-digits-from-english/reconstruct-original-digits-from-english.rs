// https://leetcode.com/problems/reconstruct-original-digits-from-english/discuss/91207/one-pass-O(n)-JAVA-Solution-Simple-and-Clear

impl Solution {
    pub fn original_digits(s: String) -> String {
        let mut count = vec![0;10];
        for c in s.chars(){
            match c {
                'z' => count[0]+=1,
                'w' => count[2]+=1,
                'x' => count[6]+=1,
                's' => count[7]+=1, //7-6
                'g' => count[8]+=1,
                'u' => count[4]+=1, 
                'f' => count[5]+=1, //5-4
                'h' => count[3]+=1, //3-8
                'i' => count[9]+=1, //9-8-5-6
                'o' => count[1]+=1, //1-0-2-4
                _ => (),
            }
        }
        count[7] -= count[6];
        count[5] -= count[4];
        count[3] -= count[8];
        count[9] = count[9] - count[8] - count[5] - count[6];
        count[1] = count[1] - count[0] - count[2] - count[4];
        let mut ans = String::new();
        for i in 0..=9{
            let c = std::char::from_digit(i as u32, 10).unwrap();
            for j in 0..count[i]{
                ans.push(c)
            }
        }
        ans
    }
}
