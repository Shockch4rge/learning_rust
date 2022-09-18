use std::collections::HashMap;

pub fn exercise1() {
    let v = vec![1, 2, 3, 4, 4, 5, 5, 5, 6, 6, 7, 7, 8, 8, 8, 8];
    
    // find median
    let middle = (v.len() / 2) as f32;
    let median = &v[middle.floor() as usize];
    
    println!("The median of the list is {:?}", median);
    
    // find mode
    let mut m = HashMap::new();

    for i in &v {
        *m.entry(i).or_insert(0) += 1;
    }

    for (key, value) in m {
        
    }
}