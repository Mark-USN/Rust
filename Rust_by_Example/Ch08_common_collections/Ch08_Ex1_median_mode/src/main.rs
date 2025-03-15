use std::collections::HashMap;

fn main() {

    let mut v1 = vec![1,2,2,3,3,3,4,4,4,4];
    let mut v2 = v1.clone(); 
    v2.extend([5,5,5,5,5]);
    let mut test3 = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut test4 = vec![0, 1, 2, 2, 3, 4, 5, 6, 7, 7, 8, 9, 10];

    println!("v1 = {:?}",v1);
    println!("v2 = {:?}",v2);
    println!("test3 = {:?}",test3);
    println!("test4 = {:?}",test4);

    println!("v1 median = {}", median(& mut v1));
    println!("v2 median = {}", median(& mut  v2));
    println!("test3 median = {}", median(& mut test3));
    println!("test4 median = {}", median(& mut test4));
    
    
    println!("v1 mode = {:?}", mode(& v1));
    println!("v2 mode = {:?}", mode(& v2));
    println!("test3 mode = {:?}", mode(& test3));
    println!("test4 mode = {:?}", mode(& test4));
}

fn median(v: & mut Vec<i32>) -> f64 {
    
    v.sort();

    let length = v.len();
    let position = length / 2;
    
    if (length & 1) != 0 {
        v[position] as f64
    } else {
        (v[position] + v[position + 1]) as f64 / 2.0
    }
}

#[derive(Debug)]
struct Modes {
    mode: i32,
    values: Vec<i32>,
 }

impl Modes {
    fn new() -> Self {
        Self {
            mode: 0,
            values: Vec::new(),
        }
    }
}


fn mode(v: & Vec<i32>) -> Modes {

    let mut modes_map = HashMap::<i32, i32>::new();
    let mut max: i32 = 0;

    for itm in v {
        let count = modes_map.entry(*itm).or_insert(0);
        *count += 1;
        if max < *count {
            max = *count;
        } 
    }

    let mut rtn_modes = Modes::new();

    rtn_modes.mode = max;

    for (key, value) in modes_map {
        if value == max {
            rtn_modes.values.push(key);
        }
    }

    rtn_modes
}
