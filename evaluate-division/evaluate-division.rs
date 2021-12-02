use std::collections::HashMap;
impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let mut map = HashMap::new();
        for i in 0..equations.len() {
            insert(&mut map, equations[i][0].clone(), equations[i][1].clone(), values[i]);
        }

        let mut ans: Vec<f64> = vec![-1.0; queries.len()];
        for i in 0..queries.len() {
            if let Some(x) = calc(&map, queries[i][0].clone(), queries[i][1].clone()){
                ans[i] = x;
            }
        }
        ans
    }
}

fn insert(map: &mut HashMap<String, HashMap<String, f64>>, x:String, y:String, val:f64) {
    if x == y{
        return;
    }
    if let Some(mx) = map.get(&x){
        if mx.contains_key(&y){
            return;
        }
    }
    // println!("{}={}*{}",x,y,val);
    
    let mut temp:Vec<(String,String,f64)> = Vec::new();
    
    if let Some(my) = map.get(&y){
        for (z,val_z) in my.iter(){
            // x=val*y, y=val_z*z
            temp.push((x.to_string(),z.to_string(),val*val_z));
        }
    }
    if let Some(mx) = map.get(&x){
        for (z,val_z) in mx.iter(){
            // y=1/val*x, x=val_z*z
            temp.push((y.to_string(),z.to_string(),1.0/val*val_z));
        }
    }
    map.entry(x.clone())
        .or_insert(HashMap::new())
        .insert(y.clone(), val);    
    map.entry(y)
        .or_insert(HashMap::new())
        .insert(x, 1.0 / val);
    
    
    for (a,b,val) in temp{
        insert(map, a, b, val);
    }
}

fn calc(map: &HashMap<String, HashMap<String, f64>>, x: String, y: String) -> Option<f64> {
    match map.get(&x){
        None => return None,
        Some(mx) => {
            match mx.get(&y){
                Some(val) => return Some(*val),
                None => (),
            }
        }
    }
    if map.get(&y).is_none(){
        return None
    }
    
    let mx = map.get(&x).unwrap();
    let my = map.get(&y).unwrap();
    for (s,val_x) in mx.iter(){
        match my.get(s){
            None => continue,
            Some(val_y) => return Some(val_x/val_y),
        }
    }
    None
}
