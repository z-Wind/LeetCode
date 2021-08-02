impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut num:i32 = 0;
        let mut before = ' ';
        for (i,c) in s.chars().enumerate(){
            match c {
                'I' =>  num+=1,
                'V' =>  {
                    if before == 'I'{
                        num+=(5-2);   
                    } else { 
                        num+=5;
                    }
                },
                'X' =>  {
                    if before == 'I'{
                        num+=(10-2);   
                    } else { 
                        num+=10;
                    }
                },
                'L' =>  {
                    if before == 'X'{
                        num+=(50-20);   
                    } else { 
                        num+=50;
                    }
                },
                'C' =>  {
                    if before == 'X'{
                        num+=(100-20);   
                    } else { 
                        num+=100;
                    }
                },
                'D' =>  {
                    if before == 'C'{
                        num+=(500-200);   
                    } else { 
                        num+=500;
                    }
                },
                'M' =>  {
                    if before == 'C'{
                        num+=(1000-200);   
                    } else { 
                        num+=1000;
                    }
                },
                _ => panic!(),
            }
            before = c;
        }
        num
    }
}