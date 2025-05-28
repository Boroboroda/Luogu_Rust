# Rust Learning

# 🦀 Rust 学习项目

　　这是一个用于学习 Rust 编程语言的个人项目。在这里，我探索 Rust 的核心概念、特性和最佳实践，同时通过实际编码练习加深理解。

　　仅供个人学习用，暂时不具备参考价值。

　　‍

# 🧰 技术栈与知识点

* 基础语法：变量、类型、函数、控制流等
* 所有权系统（Ownership）、借用（Borrowing）与生命周期（Lifetimes）
* 模块系统与包管理（Cargo）
* 错误处理（Result 与 Option）
* 并发编程（Threads、Channels 等）
* 使用标准库与第三方库（如 serde, tokio 等）（视项目而定）
* 单元测试与集成测试（视项目而定）
* 构建命令行工具 / Web 服务 / 其他应用（视项目而定）

## 🛠️ 如何运行

　　确保你已经安装了 Rust 和 Cargo。

* [ ] Rust 编译工具：[https://www.rust-lang.org/zh-CN/tools/install](https://www.rust-lang.org/zh-CN/tools/install)。
* [ ] Visual Studio Code：[https://code.visualstudio.com/Download](https://code.visualstudio.com/Download)

　　Rust 的编译工具依赖 C 语言的编译工具，这意味着你的电脑上至少已经存在一个 C 语言的编译环境。如果你使用的是 Linux 系统，往往已经具备了 GCC 或 clang。如果你使用的是 macOS，需要安装 Xcode。如果你是用的是 Windows 操作系统，你需要安装 Visual Studio 2013 或以上的环境（需要 C/C++ 支持）以使用 MSVC 或安装 MinGW + GCC 编译环境。

　　在VS Code插件中安装 rust-analyzer 和 Native Debug 两个扩展。

　　‍

## 📌 学习资源推荐

* [ ] [Rust程序设计语言(简体中文)](https://rust.bootcss.com/foreword.html)
* [ ] [2024 Rust现代实用教程](https://www.bilibili.com/video/BV15y421h7j7?vd_source=8ecea60e538416b94a4f72f1666ef720)
* [ ] [Runoob Rust教程](https://www.runoob.com/rust/rust-tutorial.html)
* [ ] ⭐[洛谷题单广场](https://www.luogu.com.cn/training/list)

　　‍

# ⭐洛谷输入示例：（仅供参考）

### 1.单行输入：

```rust
use std::io;

fn main(){
    let mut input=String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut s=input.trim().split(' ');

    let a:i32=s.next().unwrap()
               .parse().unwrap();
    let b:i32=s.next().unwrap()
               .parse().unwrap();
    println!("{}",a+b);
}
```

### 2. 多行输入：

```rust
fn main() {
    // let mut numbers = vec![];
    let mut i = 0;
    let mut numbers = vec![];
  
	loop{
        if i == 4{
            break;
        }
        let mut input=String::new();
        io::stdin().read_line(&mut input).unwrap();
        let s: Vec<_> = input.trim().split(' ').collect();

        // 将字符串解析为 f64 类型的向量
        let parsed: Vec<f64> = s.iter()
            .filter_map(|x| x.parse::<f64>().ok())
            .collect();

        // println!("{:?}",parsed);
        numbers.extend(parsed);
        // numbers.extend(s .map(|x| x.parse::<f64>().unwrap())); 
        i += 1
    }
    buy_pencil(&numbers);
}
```

　　‍
