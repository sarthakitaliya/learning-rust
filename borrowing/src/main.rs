fn main() {
    let mut s1 = String::from("hello");
    let s2 = &mut s1;
    s2.push_str(", world!");
    let s3 = &s2;
    let s4 = &s1;
    //we can't print s2 here cuz it's a mutable reference
    print!("{} ", s4);
}

// fn main() {
//     let mut s1 = String::from("hello");
//     let s2 = &mut s1;
//     let s3 = &s2;
//     let s4 = &s1;
//     //can't print s2 here cuz it's a mutable reference
//     print!("{} {}",s2, s4);
// }


// fn main() {
//     let mut s1 = String::from("hello");
//     let s2 = &mut s1;
//     let s3 = &s2;
//     let s4 = &s1;
//     // can't print s2 here cuz it's a mutable reference
//     print!("{} {} {} {}", s1, s2, s3, s4);
// }