## 变量与可变性

```rust
 let mut guess = String::new();
```


* mut 可变性
rust 需要显式声明变量可变性， 默认 immutable， 


### 常量 const

* 不允许对常量使用 mut
* 默认不能变
* 总是不能变
* 必须注明值的类型
* 只能常量表达式， 不能是运算结果


```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```



### 隐藏 Shadowing

同名变量覆盖之前的变量， 包括覆盖类型和值。 可以多次隐藏。

```rust
fn main {
    let x = 5;
    // shadowing
    let x = x + 1;
    // shadowing
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
}
```