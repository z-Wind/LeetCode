use std::collections::HashMap;
use std::collections::HashSet;
#[derive(PartialEq, PartialOrd, Eq, Clone, Hash, Debug)]
enum Rate{
    Vertical,
    Val(u32),
}
#[derive(PartialEq, PartialOrd, Eq, Clone, Hash, Debug)]
struct Line(Rate, u32);
impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        if points.len() == 1{
            return 1;
        }
        let mut line_map:HashMap<Line,HashSet<Vec<i32>>> = HashMap::new();
        for i in (0..points.len()){
            for j in (i+1..points.len()){
                let rate = match (points[i][0] - points[j][0]){
                    0 => Rate::Vertical,
                    x => Rate::Val(((points[i][1] - points[j][1]) as f32/x as f32).to_bits()),
                };
                let offset = match rate{
                    Rate::Vertical => (points[i][0] as f32).to_bits(),
                    Rate::Val(x) => {
                        // y = ax + b
                        let a = f32::from_bits(x);
                        let b = points[i][1] as f32 - a*points[i][0] as f32;
                        b.to_bits()
                    }
                };
                let line = Line(rate,offset);
                // println!("{:?},{:?} => {:?}",points[i],points[j],line);
                let mut entry = line_map.entry(line).or_insert(HashSet::new());
                entry.insert(points[i].clone());
                entry.insert(points[j].clone());
            }
        }
        line_map.values().max_by(|x, y| x.len().cmp(&y.len())).unwrap().len() as i32
    }
}
