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