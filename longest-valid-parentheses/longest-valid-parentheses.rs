use std::collections::HashMap;
use std::cmp::max;

struct LVP{
    chars:Vec<char>,
    stack:Vec<i32>,
    dp: HashMap<i32, i32>,
    maxlen: i32,
    par_num: i32,
    start:i32,
    end:i32,
}

impl LVP{
    fn new(s:&str) -> Self{
        LVP{
            chars:s.chars().collect(),
            stack:vec![],
            dp:HashMap::new(),
            maxlen:0,
            par_num:0,
            start:0,
            end:0,
        }
    }
    fn max(&mut self, len:i32){
         self.maxlen = max(self.maxlen, len);
    }
    fn longest_valid_parentheses(&mut self, i:i32) -> i32{
        //println!("i:{}, par_num:{}, dp:{:?}",i,self.par_num,self.dp);
        if i as usize == self.chars.len(){
            return -1;
        }
        
        let mut j = -1;
        match self.dp.get(&i){
            Some(&k) => {
                if k == -1{
                    return -1;
                }
                if self.par_num == 0{
                    self.end = max(self.end, k);
                }
                j = max(k, self.longest_valid_parentheses(k+1));
            },
            None => {
                match self.chars[i as usize]{
                    '(' => {
                        self.stack.push(i);
                        self.par_num += 1;
                        j = self.longest_valid_parentheses(i+1);
                        
                        // println!("( => i:{}, j:{}", i,j);
                        if j==-1 {
                            self.dp.entry(i).or_insert(j);
                        };
                    },
                    ')' => {
                        if self.par_num == 0 {
                            return i-1;
                        }
                        self.par_num -= 1;
                        
                        if self.par_num >= 0{
                            let k = self.stack.pop().unwrap();
                            self.dp.insert(k,i);
                        }
                        if self.par_num == 0{
                            self.end = max(self.end, i);
                        }
                        j = self.longest_valid_parentheses(i+1);
                    }
                    _ => panic!(),
                }
            },
        }
        
        //println!("j:{}",j);
        if self.start != self.end{
            //println!("max => start:{},end:{}",self.start,self.end);
            self.max(self.end-self.start+1);
        }
        return j;
    }
}

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut lvp = LVP::new(&s);
        for (i,c) in s.chars().enumerate(){
            if i+lvp.maxlen as usize >= lvp.chars.len(){
                break;
            }
            if c == '('{
                // if let Some(&j) = lvp.dp.get(&(i as i32)){
                //     if j == -1{
                //         println!("123");
                //         continue;       
                //     }
                // }
                lvp.par_num = 0;
                lvp.start = i as i32;
                lvp.end = i as i32;
                lvp.longest_valid_parentheses(i as i32);
                // println!("{:?} maxlen:{}", lvp.dp, lvp.maxlen);
                // println!("========================");
            }
        }
        lvp.maxlen
    }
}