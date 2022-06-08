## 使用包、Crate和模块管理不断增长的项目

Rust 有许多功能可以让你管理代码的组织，包括哪些内容可以被公开，哪些内容作为私有部分，以及程序每个作用域中的名字。这些功能。这有时被称为 “模块系统（the module system）”，包括：

* 包（Packages）： Cargo 的一个功能，它允许你构建、测试和分享 crate。
* Crates ：一个模块的树形结构，它形成了库或二进制项目。
* 模块（Modules）和 use： 允许你控制作用域和路径的私有性。
* 路径（path）：一个命名例如结构体、函数或模块等项的方式

### 包和 crate

crate 是一个二进制项或者库。crate root 是一个源文件，Rust 编译器以它为起始点，并构成你的 crate 的根模块

* cargo new --lib packageName 创建二进制库

在创建的库中 lib。rs 写入

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```

在前面我们提到了，src/main.rs 和 src/lib.rs 叫做 crate 根。之所以这样叫它们是因为这两个文件的内容都分别在 crate 模块结构的根组成了一个名为 crate 的模块，该结构被称为 模块树（module tree）。


这个树展示了一些模块是如何被嵌入到另一个模块的（例如，hosting 嵌套在 front_of_house 中）。这个树还展示了一些模块是互为 兄弟（siblings） 的，这意味着它们定义在同一模块中（hosting 和 serving 被一起定义在 front_of_house 中）。继续沿用家庭关系的比喻，如果一个模块 A 被包含在模块 B 中，我们将模块 A 称为模块 B 的 子（child），模块 B 则是模块 A 的 父（parent）。注意，整个模块树都植根于名为 crate 的隐式模块下。

crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment


## 路径用于引用模块树中的项

来看一下 Rust 如何在模块树中找到一个项的位置，我们使用路径的方式，就像在文件系统使用路径一样。如果我们想要调用一个函数，我们需要知道它的路径。

* 相对路径 ==> 从当前模块开始， 以 self、super 或当前模块的标识符开头
* 绝对路径 ==> 从 crate 根开始，以 crate 名或者字面值 crate 开头。

绝对路径和相对路径都后跟一个或多个由双冒号（::）分割的标识符。

```rust
mod front_of_house {
    // private 被访问将会编译报错
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();
}
```

## 私有

上面的示例， 我们拥有 hosting 模块和 add_to_waitlist 函数的的正确路径，但是 Rust 不让我们使用，因为它不能访问私有片段。

模块不仅对于你组织代码很有用。他们还定义了 Rust 的 私有性边界（privacy boundary）：这条界线不允许外部代码了解、调用和依赖被封装的实现细节。所以，如果你希望创建一个私有函数或结构体，你可以将其放入模块。