use std::fmt;

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

// 为了使用 `{}` 标记，必须手动为类型实现 `fmt::Display` trait。
impl fmt::Display for Person<'_> {
    // 这个 trait 要求 `fmt` 使用与下面的函数完全一致的函数签名
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 仅将 self 的第一个元素写入到给定的输出流 `f`。返回 `fmt:Result`，此
        // 结果表明操作成功或失败。注意 `write!` 的用法和 `println!` 很相似。
        write!(f, "{} {}", self.age, self.name)
    }
}

// 定义一个包含单个 `Vec` 的结构体 `List`。
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 使用元组的下标获取值，并创建一个 `vec` 的引用。
        let vec = &self.0;

        write!(f, "[")?;

        // 使用 `v` 对 `vec` 进行迭代，并用 `count` 记录迭代次数。
        for (count, v) in vec.iter().enumerate() {
            // 对每个元素（第一个元素除外）加上逗号。
            // 使用 `?` 或 `try!` 来返回错误。
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}:", count)?;
            write!(f, "{}", v)?;
        }

        // 加上配对中括号，并返回一个 fmt::Result 值。
        write!(f, "]")
    }
}

struct FreeDesign {
    age: u8,
}

impl fmt::Display for FreeDesign {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.age)
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:X}", self.green);
        write!(f, "0x{:X}", self.red);
        write!(f, "0x{:X}", self.blue)
    }
}

pub fn main() {
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    let free = FreeDesign { age };

    let list = List(vec![1, 23, 3]);

    for color in [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
    ]
    .iter()
    {
        // 在添加了针对 fmt::Display 的实现后，请改用 {} 检验效果。
        println!("{}", color)
    }

    // 美化打印
    println!("{}", peter);
    println!("{}", free);
    println!("{}", list);
}
