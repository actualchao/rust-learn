pub fn _tuples() {
    let test_tuples = (1, "123");
    println!("{}", test_tuples.0);

    // 元祖可以成为元祖的成员
    let _test_tuples_memnt = ((1, "123"), 1, 3u32);

    println!("tuple of tuples {:?}", test_tuples);

    // too long tuples can not be print
    // let too_long_tuples = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("tuple of tuples {:?}", too_long_tuples);

    // 创建单元素元组需要一个额外的逗号，这是为了和被括号包含的字面量作区分。
    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    // tuples 能够使用解构
    let (a, b) = test_tuples;
    
}
