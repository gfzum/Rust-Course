# Rust-Course

Personal Rust learning use, from [Rust Course](https://course.rs/about-book.html).

## Basic

暂时不用的变量可以在变量名前加 `_` 以规避编译器提醒，或者在函数前添加 `#[allow(unused_variables)]`。

结构式复制和变量匹配。

```rust
struct Struct {
    e: i32
}

fn main(){
    let (mut a, b, c, d, e);
    (a, b) = (1, 2);
    // _ 代表匹配一个值，但是我们不关心具体的值是什么，因此没有使用一个变量名而是使用了 _
    [c, .., d, _] = [1, 2, 3, 4, 5];
    Struct { e, .. } = Struct { e: 5 };
    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);
}
```

Rust 没有隐式类型转换，只能显示的进行转换。例如如果我们想要进行 `f64` 类型数据和 `i32` 类型数据的加法，可以通过以下两种方式进行：

```rust
let x = 5;
let y = 0.1;

println!("{}", y + x as f64)
println!("{}", y + f64::from(x))
```

在下面这个变量遮蔽（shadowing）的例子中，两个 spaces 的类型不一样，这也是无法使用 `mut` 变量进行相同操作的原因和遮蔽的方便之处。

```rust
// 字符串类型
let spaces = "   ";
// usize数值类型
let spaces = spaces.len();
```

`main` 函数和 `println!()` 不是无返回值，而是返回单元类型 `()` ，作为一个值用来占位，但是完全不占用任何内存。无返回值在 Rust 中被定义为 `发散函数（diverge function）`。单元类型 `()` 也可以作为 `map` 的 value，表示只关心 key。

### Statements and Expressions

- 有返回值的就是表达式（expression），如果没有返回值会隐式返回一个 `()`
- 表达式后加上分号就变成了一条语句（statement），没有返回值。`let x = 1` 和 `x += 2` 都是一条语句
- 使用语句块赋值的时候，记得 `{}` 后的分号，因为这仍然是一条语句

```rust
let y = {
    let x = 3;
    x + 1
};
println!("The value of y is: {}", y);
```

有关函数

- 函数是一种表达式
- 函数的返回值就是函数体最后一条表达式的返回值，也可以提前用 `return` 返回
- 函数没有返回值或通过 `;` 结尾，那么返回一个 `()`，其他时候必须要指明返回类型
- 使用 `!` 作为函数返回类型，表示该函数永不返回（diverge function）

```rust
fn clear(text: &mut String) -> () {
    *text = String::from("");
}

fn dead_end() -> ! {
    panic!("你已经到了穷途末路，崩溃吧！");
    //todo!();
    //unimplemented!()
}
```

## Ownership
