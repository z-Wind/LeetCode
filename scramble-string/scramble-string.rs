impl Solution {
    pub fn is_scramble(s1: String, mut s2: String) -> bool {
        if s1 == s2{
            return true;
        }
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        
        is_scramble(s1, s2)
    }
}

fn is_scramble(s1: &[u8], s2: &[u8]) -> bool{
    //println!("s1:{:?}\ns2:{:?}",String::from_utf8(s1.to_vec()).unwrap(),String::from_utf8(s2.to_vec()).unwrap());
    if s1 == s2{
        return true;
    }
    
    let mut map1 = [0;26];
    let mut map2 = [0;26];
    let mut max_i = usize::MAX;
    for i in (0..s1.len()-1){
        map1[(s1[i]-b'a') as usize] += 1;
        map2[(s2[i]-b'a') as usize] += 1;
        if map1 == map2{
            //println!("forward:{}\nmap1:{:?}\nmap2:{:?}", i, map1, map2);
            max_i = i;
        }
    }
    if max_i != usize::MAX && is_scramble(&s1[..max_i+1], &s2[..max_i+1]) && is_scramble(&s1[max_i+1..], &s2[max_i+1..]){
        return true;
    }
    
    map1 = [0;26];
    map2 = [0;26];
    max_i = usize::MAX;
    for i in (0..s1.len()-1){
        let j = s2.len()-1-i;
        map1[(s1[i]-b'a') as usize] += 1;
        map2[(s2[j]-b'a') as usize] += 1;
        if map1 == map2{
            //println!("backward:{}\nmap1:{:?}\nmap2:{:?}", i, map1, map2);
            max_i = i;
        }
    }
    let j = s2.len()-1-max_i;
    if max_i != usize::MAX && is_scramble(&s1[..max_i+1], &s2[j..s2.len()]) && is_scramble(&s1[max_i+1..], &s2[0..j]){
            return true;
        }
    
    false
}