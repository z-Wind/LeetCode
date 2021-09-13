//https://leetcode.com/problems/max-points-on-a-line/discuss/47113/A-java-solution-with-notes
use std::collections::HashMap;
impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        if (points.is_empty()) { return 0 };
        if (points.len()<=2) { return points.len() as i32 };
        
        let mut map = HashMap::new();
        let mut result=0;
        for i in (0..points.len()){ 
            map.clear();
            let mut overlap=0;
            let mut max=0;
            for j in (i+1..points.len()){
                let mut x=points[j][0]-points[i][0];
                let mut y=points[j][1]-points[i][1];
                if (x==0 && y==0){
                    overlap+=1;
                    continue;
                }
                let gcd=generateGCD(x,y);
                if (gcd!=0){
                    x/=gcd;
                    y/=gcd;
                }
                
                let entry = map.entry((x,y)).or_insert(0);
                *entry += 1;
                max=max.max(*entry);
            }
            result=result.max(max+overlap+1);
        }
        result
    }
}

fn generateGCD(a:i32,b:i32) -> i32{
    if (b==0) {a}
    else {generateGCD(b,a%b)}
}
