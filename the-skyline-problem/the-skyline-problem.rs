// https://briangordon.github.io/2014/08/the-skyline-problem.html
// https://zhuanlan.zhihu.com/p/48403793
// https://leetcode.com/problems/the-skyline-problem/discuss/61197/(Guaranteed)-Really-Detailed-and-Good-(Perfect)-Explanation-of-The-Skyline-Problem

use std::cmp::Ordering;
use std::collections::BTreeMap;

#[derive(Debug)]
struct Point{
    x:i32,
    y:i32,
    is_start:bool,
}
impl Solution {
    pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {        
        let mut points:Vec<Point> = Vec::new();
        for b in buildings{
            points.push(Point{x:b[0],y:b[2],is_start:true});
            points.push(Point{x:b[1],y:b[2],is_start:false});
        }
        points.sort_unstable_by(|a, b| sort_cmp(a,b));
        // println!("{:?}", points);
        
        let mut ans:Vec<Vec<i32>> = Vec::new();
        let mut heap = BTreeMap::new();
        *heap.entry(0).or_insert(0) += 1;
        let mut pre_h = 0;
        for p in points{
            // println!("{}: {:?}",pre_h, p);
            if p.is_start{
                *heap.entry(p.y).or_insert(0) += 1;
            } else {
                let mut entry = heap.entry(p.y).or_insert(0);
                *entry -= 1;
                if *entry == 0{
                    heap.remove(&p.y);
                }
            }
            if let Some(&h) = heap.keys().last(){
                if pre_h != h{
                    ans.push(vec![p.x,h]);
                    pre_h = h;
                }
            }
            // println!("{:?}",ans);
        }
        ans
    }
}

fn sort_cmp(a:&Point, b:&Point) -> Ordering{
    match (a.is_start, b.is_start, a.x.cmp(&b.x), a.y.cmp(&b.y)){        
        (_, _, Ordering::Less, _) => Ordering::Less,
        (_, _, Ordering::Greater, _) => Ordering::Greater,
        
        (true, false, Ordering::Equal, _) => Ordering::Less,
        (false, true, Ordering::Equal, _) => Ordering::Greater,
        
        (true, true, Ordering::Equal, Ordering::Less) => Ordering::Greater,
        (true, true, Ordering::Equal, Ordering::Greater) => Ordering::Less,
        (true, true, Ordering::Equal, Ordering::Equal) => Ordering::Equal,
        
        (false, false, Ordering::Equal, Ordering::Less) => Ordering::Less,
        (false, false, Ordering::Equal, Ordering::Greater) => Ordering::Greater,
        (false, false, Ordering::Equal, Ordering::Equal) => Ordering::Equal,
    }
}