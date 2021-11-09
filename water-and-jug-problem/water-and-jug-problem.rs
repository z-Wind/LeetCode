use std::collections::HashSet;
impl Solution {
    pub fn can_measure_water(jug1_capacity: i32, jug2_capacity: i32, target_capacity: i32) -> bool {
        if jug1_capacity + jug2_capacity < target_capacity {
            return false;
        }
        let mut set = HashSet::new();
        can_measure_water(
            &mut set,
            jug1_capacity,
            jug1_capacity,
            jug2_capacity,
            jug2_capacity,
            target_capacity,
        )
    }
}

fn can_measure_water(set:&mut HashSet<(i32,i32)>, j1_max: i32, j1_r: i32, j2_max: i32, j2_r: i32, target: i32) -> bool {
    if set.contains(&(j1_r,j2_r)){
        return false;
    }
    set.insert((j1_r,j2_r));
    
    // println!("{},{}",j1_r,j2_r);
    if j1_r < 0 || j2_r < 0 || j1_r > j1_max || j2_r > j2_max{
        return false;
    }
    if j1_r + j2_r == target || j1_r == target || j2_r == target {
        return true;
    }
    // put water from jug2 to jug1
    can_measure_water(set, j1_max, j1_max, j2_max, j2_r-j1_max, target)
    || can_measure_water(set, j1_max, j2_r, j2_max, j2_max, target)
    || can_measure_water(set, j1_max, j1_max, j2_max, j2_max-(j1_max-j1_r), target)
    // put water from jug1 to jug2
    || can_measure_water(set, j1_max, j1_r-j2_max, j2_max, j2_max, target)
    || can_measure_water(set, j1_max, j1_max, j2_max, j1_r, target)
    || can_measure_water(set, j1_max, j1_max-(j2_max-j2_r), j2_max, j2_max, target)
}
