impl Solution {
    pub fn find_radius(mut houses: Vec<i32>, mut heaters: Vec<i32>) -> i32 {
        let m = houses.len();
        let n = heaters.len();
        
        houses.sort_unstable();
        heaters.sort_unstable();
        
        let mut i = 0;
        let mut j = 0;
        let mut radius = 0;
        let mut h1 = None;
        let mut h2 = Some(heaters[j]);
        while i < m {
            // println!("{}: h1:{:?}, h2:{:?}, radius:{}", houses[i], h1, h2, radius);
            if h1 <= Some(houses[i]) && Some(houses[i]) <= h2{
                let r = match (h1, h2) {
                    (None, Some(x)) => x-houses[i],
                    (Some(x1), Some(x2)) => (houses[i]-x1).min(x2-houses[i]),
                    x => panic!("{:?}", x),
                };
                radius = radius.max(r);
            } else if Some(houses[i]) > h2 {
                h1 = h2;
                j += 1;
                h2 = Some(if j == n {
                    i32::MAX
                } else {
                    heaters[j]
                });
                continue;
            }
            i += 1;
        }
        
        radius
    }
}