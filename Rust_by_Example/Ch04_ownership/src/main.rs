fn main() {
//    let mut s = String::from("hello");

//    change(&s);

    //let r1 = &s;
    //let r2 = & s;
    //let r3 = & s;

    //println!("r1 is {r1} r2 is {r2} r3 is {r3}");

    //let reference_to_nothing = dangle();

    //let mut s = String::from("hello world");
    
    //let word = first_word(&s);
  
    //s.clear();

    //println!("The first word is: {word}.");

    let a = [1,2,3,4,5];
    let slice = &a[1..3];

    println!("slice lenght = {}",slice.len());
    println!("slice = [{}, {}]", slice[0], slice[1]);

    assert_eq!(slice, &[2, 3]);

}

//fn change(some_string: & String) {
//    some_string.push_str(", world");
//}

//fn dangle () -> String {
    
//    let s = String::from("hello");

//    s
//}


//fn first_word(s: &String) -> usize {
//    let bytes = s.as_bytes();

//    for (i, &item) in bytes.iter().enumerate() {
//        if item == b' ' {
//            return i;
//        }
//    }
//    s.len()
//}

//fn first_word(s: &String) -> &str {
//    let bytes = s.as_bytes();

//    for (i, &item) in bytes.iter().enumerate() {
//        if item == b' ' {
//            return &s[0..i];
//        }
//    }
//    &s[..]
//}