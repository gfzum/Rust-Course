# Rust-Course

Personal Rust learning use

## Basic

Rust 没有隐式类型转换，只能显示的进行转换。例如如果我们想要进行 f64 类型数据和 i32 类型数据的加法，可以通过以下两种方式进行：

```rust
let x = 5;
let y = 0.1;

println!("{}", y + x as f64)
println!("{}", y + f64::from(x))
```

在这个变量遮蔽（shadowing）的例子中，两个 spaces 的类型不一样，这也是无法使用 `mut` 变量进行相同操作的原因和遮蔽的方便之处。

```rust
fn main() {
// 字符串类型
let spaces = "   ";
// usize数值类型
let spaces = spaces.len();
```

`main` 函数和 `println!()` 不是无返回值，而是返回单元类型 `()` ，作为一个值用来占位，但是完全不占用任何内存。无返回值在 Rust 中被定义为 `发散函数（diverge function）`。单元类型 `()` 也可以作为 `map` 的 value，表示只关心 key。

### Statements and Expressions

- 有返回值的就是表达式（expression），如果没有返回值会隐式返回一个 `()`
- 表达式后加上分号就变成了一条语句（statement），没有返回值。`let x = 1` 也是一条语句

有关函数

- 函数是一种表达式
- 函数的返回值就是函数体最后一条表达式的返回值，也可以提前用 `return` 返回
- 函数没有返回值或通过 `;` 结尾，那么返回一个 `()`
- 使用 `!` 作为函数返回类型，表示该函数永不返回（diverge function）

```rust
fn clear(text: &mut String) -> () {
  *text = String::from("");
}

fn dead_end() -> ! {
  panic!("你已经到了穷途末路，崩溃吧！");
}
```
