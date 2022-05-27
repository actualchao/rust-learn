# 引用与借用

为了避免一个值在程序执行过程中需要不同的在函数中转移所有权的情况， 我们可以提供一个值的引用（reference）， 它就像一个指针， 因为是一个地址, keyifangwen 存储在地址的其他变量的数据。与指针不同，引用确保指向某个特定类型的有效值。
** 引用与允许使用值但不获取其所有权**

```rust
fn main() {
    let s1 = String::from("hello");

    // &s1 传递引用
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    // & 符号接受引用
    s.len()
}
```



tips： 解引用，与使用 & 引用相反的操作是 解引用（dereferencing），它使用解引用运算符，*

变量 s 有效的作用域与函数参数的作用域一样，不过当 s 停止使用时并不丢弃引用指向的数据，因为 s 并没有所有权。当函数使用引用而不是实际值作为参数，无需返回值来交还所有权，因为就不曾拥有所有权。

我们将创建一个引用的行为称为 借用（borrowing）

* 不能修改引用变量的值（下面有可变引用）


### 可变引用

通过添加 &mut 创建可变引用

```rust
fn main() {
    let mut s = String::from("hello");

    // mut 可更改
    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

* 在同一时间只能有一个对某一特定数据的可变引用。
* 不能在拥有不可变引用的同时拥有可变引用
```rust 
    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    let r3 = &mut s; // 大问题
```


## 悬垂引用（Dangling References）
在具有指针的语言中，很容易通过释放内存时保留指向它的指针而错误地生成一个 悬垂指针（dangling pointer），所谓悬垂指针是其指向的内存可能已经被分配给其它持有者。相比之下，在 Rust 中编译器确保引用永远也不会变成悬垂状态：当你拥有一些数据的引用，编译器确保数据不会在其引用之前离开作用域。

```rust

fn main() {
    let reference_to_nothing = dangle();
}


fn dangle() -> &String { // dangle 返回一个字符串的引用

    let s = String::from("hello"); // s 是一个新字符串

    &s // 返回字符串 s 的引用
} // 这里 s 离开作用域并被丢弃。其内存被释放。
  // 危险！ 

```

引用的规则

* 在任意给定时间， 要么只能有一个可变引用， 要么只能由一个或多个不可变引用
* 引用必须总是有效的。