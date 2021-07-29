// Manacher’s Algorithm
// https://havincy.github.io/blog/post/ManacherAlgorithm/
// https://medium.com/hoskiss-stand/manacher-299cf75db97e

fn insert_symbol_in_string(mut s:String) -> String {    
    for i in (0..=s.len()).rev() {
        s.insert(i, '_');
    }
    
    format!("^{}$", s)
}

fn radius_string(s: &str) -> (Vec<usize>, usize){
    let mut max_index = 0;
    let mut v: Vec<usize> = Vec::with_capacity(s.len());
    v.push(0); //^
    v.push(0); //_
    
    let chars:Vec<char> = s.chars().collect();
    let (mut center,mut center_R) = (0,0);
    for i in (2..s.len()-1){
        let mut r = 0;
        let mirror_i = center-(i-center);
        if i>=center_R || 
           mirror_i <= 2 || 
           i + v[mirror_i] >= center_R{
            // calculate radius
            while chars[i-r-1] == chars[i+r+1]{
                r+=1
            }
            
            center = i;
            center_R = i+r;
        } else {
            r = v[mirror_i];
        }
        v.push(r);
        
        if v[max_index] < r{
            max_index = i;
        }
    }
    
    (v, max_index)
}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let si = insert_symbol_in_string(s.clone());
        let (v, max_index) = radius_string(&si);
        
        //println!("{}\n{}\n{:?}\n{}\n", s, si, v, max_index);
        let r = v[max_index];
        let start = (max_index-r)/2;
        return String::from(&s[start..start+r])
    }
}