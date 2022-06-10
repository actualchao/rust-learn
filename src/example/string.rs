pub fn string() {
    crate::dev_log("string start");

    // 创建
    let mut string = String::new();
    // string = "hello".to_string();
    println!("string: {}", string);
    string = String::from("hello2");
    println!("string: {}", string);

    // all 有效 utf-8
    // let hello = String::from("السلام عليكم");
    // let hello = String::from("Dobrý den");
    // let hello = String::from("Hello");
    // let hello = String::from("שָׁלוֹם");
    // let hello = String::from("नमस्ते");
    // let hello = String::from("こんにちは");
    // let hello = String::from("안녕하세요");
    // let hello = String::from("你好");
    // let hello = String::from("Olá");
    // let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
    println!("string: {}", hello);

    // 更新
    string.push_str("123");
    string.push('1'); // 单独字符
    let s2 = "bar";
    string.push_str(s2);
    println!("s2 is {}", s2);
    println!("string: {}", string);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用
    println!("s3 is {}", s3);


    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s is {}", s);

    crate::dev_log("string end");
}
