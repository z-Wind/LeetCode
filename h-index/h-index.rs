impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let n = citations.len();
        let mut h_count = vec![0;n+1];
        for c in citations{
            for i in 0..=n.min(c as usize){
                h_count[i] += 1;   
            }
        }
        // println!("{:?}", h_count);
        h_count.iter().enumerate().rposition(|(i, &count)| count>=i as i32 ).unwrap() as i32
    }
}