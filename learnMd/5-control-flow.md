# 控制流 control flow


### if
```rust
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
```

* condition 不需要包裹 （）
* condition must be boolean
* else if
* let number = if condition { 5 } else { 6 };
* 赋值时使用 if 必须保证分支值类型一致
* 

### loop

```rust
fn main() {
    loop {
        // do something
    }
}
```

* 循环 label： 使用关键之标记循环， 在嵌套循环中可以 将标签与 break 或 continue 一起使用，使这些关键字应用于已标记的循环而不是最内层的循环
* 循环lebel 命名规则 ： 'some_name

```rust
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
}

// log
// remaining = 10
// remaining = 9
// count = 1
// remaining = 10
// remaining = 9
// count = 2
// remaining = 10
// End count = 2
```


### 循环返回值

返回值加入你用来停止循环的 break 表达式，它会被停止的循环返回：

```rust
fn main() {
    let mut number = 0;
    loop {
        number += 10

        if number == 20 {
            break number * 2;
        }
    }
}
````


## while 循环

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```


## for 循环

```rust
fn main() {
    let a = [1,2,3,4];

    // let element in a
    for element in a {
        println!("the value is: {}", element);
    }
}
```
