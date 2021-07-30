// 從第 n 層下去，回到第 n 層，共需 2*(num_rows-n)
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1{
            return s
        }
        
        let num_rows = num_rows as usize;
        
        let chars:Vec<char> = s.clone().chars().collect();
        let mut ans:Vec<char> = s.clone().chars().collect();
        
        
        let mut i = 0;
        for n in (1..=num_rows){
            let mut i_to = n;    
            let mut shift:usize;
            let mut i_shift=1;
            while i < s.len(){
                //println!("{}:{}",n,i_to);
                ans[i] = chars[i_to-1];
                
                i+=1;
                
                if (i_shift%2 == 0 && n != 1) || n == num_rows{
                    shift = 2*(num_rows-1) - 2*(num_rows-n);
                } else {
                    shift = 2*(num_rows-n);
                }

                i_to += shift;
                i_shift += 1;

                if i_to > s.len(){
                    break;
                }
            }
        }
        
        
        ans.into_iter().collect()
    }
}