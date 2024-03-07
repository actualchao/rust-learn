use std::fmt;


#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}



// 为了使用 `{}` 标记，必须手动为类型实现 `fmt::Display` trait。
impl fmt::Display for Person<'_> {
    // 这个 trait 要求 `fmt` 使用与下面的函数完全一致的函数签名
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 仅将 self 的第一个元素写入到给定的输出流 `f`。返回 `fmt:Result`，此
        // 结果表明操作成功或失败。注意 `write!` 的用法和 `println!` 很相似。
        write!(f, "{}", self.age)
    }
}

fn main() {
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // 美化打印
    println!("{}", peter);
}
