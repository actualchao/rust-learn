// 这里是引用关系的范例 

pub fn reference () {
    let mut s = String::from("hello world");

    // let s1 = &s;
    // let s2 = &s;

    let s3 = &mut s;

    println!("{}, and ", s3);
}