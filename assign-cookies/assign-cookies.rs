impl Solution {
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        let g_n = g.len();
        let s_n = s.len();
        if s_n == 0{
            return 0;
        }
        
        g.sort_unstable();
        s.sort_unstable();
        
        // println!("g:{:?}", g);
        // println!("s:{:?}", s);
        
        let mut count = 0;
        let mut i = 0;
        let mut j = 0;
        while i < g_n && j < s_n{
            if g[i] <= s[j]{
                count += 1;
                i += 1;
            }
            j += 1;
        }
        
        count
    }
}