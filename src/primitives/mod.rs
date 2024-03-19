mod tuples;
use tuples::_tuples;

pub fn primitives() {
    // let apples = 5;
    // let panios = 6;

    // print!("{}", apples);
    // print!("{}", panios);

    // _scalar_type()
    _tuples()

}

fn _scalar_type() {
    let _num1: i32 = 2147483647;
    let _num2: u32 = 2147483648;

    // &str 类型
    let mut _str: &str = "test string";
    // string 类型
    let mut _str1: String = "test".to_string();

    // default is i32, 默认推到类型为i32
    let _default_integer = 7;
    let _default_float = 1.3;

    // 后缀制定类型
    let _suffix_float = 1.3f32;

    // 可变的
    let mut _mutable_num = 32i32;
    _mutable_num = 34;

    // will be error
    // mutable_num = true;

    let mutable_num = true;

    print!("{}", mutable_num);
    print!("\n");

    // 整数相减
    println!("1 - 2 = {}", 1i32 - 2);
}
