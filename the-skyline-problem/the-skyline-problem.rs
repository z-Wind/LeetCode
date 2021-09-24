use std::cmp::Reverse;
impl Solution {
    pub fn get_skyline(mut buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {        
        buildings.sort_by_key(|x| x[2]);
        // println!(" b: {:?}",buildings);
        let mut new_buildings:Vec<Vec<i32>> = Vec::new();
        'outer: while !buildings.is_empty(){
            let mut b = buildings.pop().unwrap();
            for i in (0..new_buildings.len()){
                let nb = &mut new_buildings[i];
                // println!(" b:{:?}\nnb:{:?}",b,nb);
                // same height
                if b[2] == nb[2] && 
                   (nb[1] >= b[0] && b[0] >= nb[0] || 
                    nb[1] >= b[1] && b[1] >= nb[0]){
                    nb[0] = nb[0].min(b[0]);
                    nb[1] = nb[1].max(b[1]);
                    continue 'outer; 
                }
                // no intersection
                if (b[0] <= nb[0] && b[1] <= nb[0]) ||
                   (b[0] >= nb[1] && b[1] >= nb[1]){
                       continue;
                }
                // include
                if nb[1] >= b[0] && b[0] >= nb[0] && 
                   nb[1] >= b[1] && b[1] >= nb[0]{
                   continue 'outer; 
                }
                // intersection
                if b[0] < nb[0] && nb[1] >= b[1] && b[1] > nb[0]{ // left
                    b[1] = nb[0];
                } else if b[1] > nb[1] && nb[1] > b[0] && b[0] >= nb[0]{ // right
                    b[0] = nb[1];
                } else if b[1] > nb[1] && b[0] < nb[0]{ // middle
                    buildings.push(vec![b[0], nb[0], b[2]]);
                    buildings.push(vec![nb[1], b[1], b[2]]);
                    continue 'outer; 
                }
            }
            new_buildings.push(b);
        }
        new_buildings.sort_by_key(|x| x[0]);
        // println!("nb: {:?}",new_buildings);
        
        let mut points:Vec<Vec<i32>> = Vec::new();
        for w in new_buildings.windows(2){
            if w[0][1] == w[1][0]{
                points.push(vec![w[0][0], w[0][2]]);
            } else {
                points.push(vec![w[0][0], w[0][2]]);
                points.push(vec![w[0][1], 0]);
            }
        }
        let n = new_buildings.len() - 1;
        points.push(vec![new_buildings[n][0], new_buildings[n][2]]);
        points.push(vec![new_buildings[n][1], 0]);
        points
    }
}