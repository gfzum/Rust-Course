# Rust-Course

Personal Rust learning use, from [Rust Course](https://course.rs/about-book.html).

## Basic

暂时不用的变量可以在变量名前加 `_` 以规避编译器提醒，或者在函数前添加 `#[allow(unused_variables)]`。

结构式复制和模式匹配：

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

- 基本类型（包括指针不可变引用 `&T`）存储在栈上，并通过自动拷贝的方式来赋值，实现了 `copy trait`
- 对于大小未知或者可能变化的数据，将在堆上分配内存存储，并返回一个表示位置地址的指针
- 处理器处理和分配在栈上数据会比在堆上的数据更加高效
- 当所有权转移时，可变性也可以随之改变，如 `let mut s1 = s`

```rust
// copy trait
let x = 5; // 变量绑定
let y = x; // 浅拷贝

let x: &str = "hello, world";
let y = x; // 引用拷贝，x y 引用同一个字符串

// drop trait ，将 String 的所有权从 s1 转交给 s2 ，s1 失效
let s1 = String::from("hello");
let s2 = s1; // move
println!("{}, world!", s1); // error, value used after move

```

上面这个堆上字符串所有权转换的过程类似浅拷贝 + 令原变量失效，被称为移动（move）。Rust 永远也不会自动创建数据的深拷贝，任何自动的复制都不是深拷贝。深拷贝可以使用 `s2 = s1.clone()` 方法。

如果要在堆中分配原来在栈上分配的变量，需要用 [BOX](https://www.bookstack.cn/read/rust-notes/memory-safety.md) 来构造。

部分 move：当解构一个变量时，可以同时使用 move 和引用模式绑定的方式，变量中一部分的所有权被转移给其它变量，而另一部分我们获取了它的引用。

```rust
fn main() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    // 通过这种解构式模式匹配，person.name 的所有权被转移给新的变量 `name`
    // 但是，这里 `age` 变量却是对 person.age 的引用, 这里 ref 的使用相当于: let age = &person.age 
    let Person { name, ref age } = person;

    println!("The person's age is {}", age);
    println!("The person's name is {}", name);

    // Error! 原因是 person 的一部分已经被转移了所有权，因此我们无法再使用它
    //println!("The person struct is {:?}", person);

    // 虽然 `person` 作为一个整体无法再被使用，但是 `person.age` 依然可以使用
    println!("The person's age from person struct is {}", person.age);
}

```

### function

将值传递给函数，一样会发生 `move` 或者 `copy`。

```rust
fn main() {
    let s = String::from("hello");  // s 进入作用域
    takes_ownership(s);             // s 的值移动到函数里，所以到这里不再有效
    let x = 5;                      // x 进入作用域
    makes_copy(x);                  // x 应该移动函数里，但 i32 是 Copy 的，所以在后面可继续使用 x

} // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，所以不会有特殊操作

fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作
```

```rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership 将返回值移给 s1
    let s2 = String::from("hello");     // s2 进入作用域
    let s3 = takes_and_gives_back(s2);  // s2 被移动到takes_and_gives_back 中，它也将返回值移给 s3
} // 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，所以什么也不会发生。s1 移出作用域并被丢弃

fn gives_ownership() -> String {             // gives_ownership 将返回值移动给调用它的函数
    let some_string = String::from("hello"); // some_string 进入作用域.
    some_string                              // 返回 some_string 并移出给调用的函数
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域
    a_string  // 返回 a_string 并移出给调用的函数
}
```

不断转交所有权比较麻烦，可以使用引用解决。

### Borrowing

- 常规引用是一个指针类型，指向了对象存储的内存地址，允许使用值但是不获取所有权
- 使用解引用运算符 `*` 解出引用所指向的值
- 默认不可变

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // 这里，s 离开了作用域。但因为它并不拥有引用值的所有权，所以什么也不会发生
```

- 只能对可变对象进行可变引用
- borrow checker：同一作用域下，特定数据只能粗在一个可变引用，避免数据竞争
- 可变引用与不可变引用不能同时存在

```rust
fn main() {
    let mut s = String::from("hello"); // 必须 mut 且为 String，&str 没有 push_str 方法
    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

Non-Lexical Lifetimes(NLL)：引用的作用域从创建开始，一直持续到它最后一次使用的地方，和变量的作用域不一样（到某一个 `}` ）

```rust
fn main() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // 新编译器中，r1,r2作用域在这里结束

    let r3 = &mut s;
    println!("{}", r3);
} // 老编译器中，r1、r2、r3作用域在这里结束
  // 新编译器中，r3作用域在这里结束

```

总结

- 同一时刻（引用作用域），你只能拥有要么一个可变引用, 要么任意多个不可变引用
- 引用必须总是有效的（不会出现悬垂引用）
- `ref` 与 `&` 类似，可以用来获取一个值的引用，但是它们的用法有所不同

## String

### Slices

字符串切片 `&str` ：对 `String` 类型中某一部分的引用。如果无法保证索引的字节刚好落在字符的边界上，会引起程序崩溃。

```rust
let s = String::from("hello world");
let hello = &s[0..5];
let world = &s[6..11];
```

其他切片，如数组，也是一样的。如果使用切片时不加 `&` 会有 size cannot be known 的报错，因为切片的长度无法在编译期得知，因此无法直接使用切片类型。切片类型 `[T]` 拥有不固定的大小，而切片引用类型 `&[T]` 则具有固定的大小，因为 Rust 很多时候都需要固定大小数据类型，因此 `&[T]` 更有用。

- 一个切片引用占用了2个字大小的内存空间，字的大小取决于处理器架构（x86-64下8字节）
- 第一个字是指向数据的指针，第二个字是切片的长度。

```rust
let a = [1, 2, 3, 4, 5];
let slice = &a[1..3];
assert_eq!(slice, &[2, 3]);
```

字符串字面量是切片，该切片指向了程序可执行文件中被硬编码的文本（大小未知）的某个点。字符串字面量是不可变的，因为 `&str` 是一个不可变引用。

```rust
let s1 = "hello" // 此处 s 类型为 &str ，是不可变的字符串字面值，存储在？
let mut s2 = String::from("hello"); // 此处 s2 为动态字符串类型，分配在堆上，可以进行修改
s2.push_str(" world!");
```

### String 类型

[干货描述](https://course.rs/basic/compound-type/string-slice.html#%E4%BB%80%E4%B9%88%E6%98%AF%E5%AD%97%E7%AC%A6%E4%B8%B2)，以及 [字符串的操作](https://course.rs/basic/compound-type/string-slice.html#%E6%93%8D%E4%BD%9C%E5%AD%97%E7%AC%A6%E4%B8%B2)。

- 事实上 String 是一个智能指针，它作为一个结构体存储在栈上，然后指向存储在堆上的字符串底层数据
- 存储在栈上的智能指针结构体由三部分组成：一个指针只指向堆上的字节数组、已分配的容量 capacity、已使用的长度（已使用的长度小于等于已分配的容量，当容量不够时，会重新分配内存空间）
- 不允许 `s[0]` 这样的字符串索引，和 utf-8 的实现方式以及性能表现有关，可以用切片 `&s[0..1]`
- 变量在离开作用域后，自动调用 `drop` 释放其占用的内存

和 `&str` 的转换

```rust
fn main(){
    let s = "hello".to_string();

    let s = String::from("hello,world!");
    say_hello(&s);
    say_hello(&s[..]);
    say_hello(s.as_str());
}

fn say_hello(s: &str) {
    println!("{}",s);
}
```

用 `+` 连接字符串的时候，如果想保留第一个字符串的所有权，可以写 `s3 = s1.clone() + &s2`

- `&s[..]` 和 `&s` 的区别？
- `&str` 和 `&String` 的区别？
- `deref trait`

raw string

```rust
let raw_str = "Escapes don't work here: \x3F \u{211D}";
// modify above line to make it work
assert_eq!(raw_str, "Escapes don't work here: ? ℝ");

// If you need quotes in a raw string, add a pair of #s
let quotes = r#"And then I said: "There is no escape!""#;
println!("{}", quotes);

// If you need "# in your string, just use more #s in the delimiter.
// You can use up to 65535 #s.
let  delimiter = r###"A string with "# in it. And even "##!"###;
println!("{}", delimiter);

// Fill the blank
let long_delimiter = r###"Hello, "##""###;
assert_eq!(long_delimiter, "Hello, \"##\"")
```

## Compound types

### tuple & struct

元组：多种类型组合到一起，长度固定，顺序固定。可以用模式匹配或 `.` 获取值。

```rust
fn main() {
    let t: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, _) = t;
    let z = t.2;
}

```

struct

- standard
- tuple struct
- unit struct

`dbg!` 会拿走表达式的所有权，打印出相应的文件名、行号、表达式结果等 debug 信息（在 stderr 中），并把表达式值的所有权返回。

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}
```

### Enum

option

### Array

```rust
fn main() {
  // 编译器自动推导出one的类型（根据后面的 arrays）
  let one             = [1, 2, 3];
  // 显式类型标注
  let two: [u8; 3]    = [1, 2, 3];
  let blank1          = [0; 3];
  let blank2: [u8; 3] = [0; 3];

  // arrays是一个二维数组，其中每一个元素都是一个数组，元素类型是[u8; 3]
  let arrays: [[u8; 3]; 4]  = [one, two, blank1, blank2];

  // 借用arrays的元素用作循环中
  for a in &arrays {
    print!("{:?}: ", a);
    // 将a变成一个迭代器，用于循环
    // 你也可以直接用for n in a {}来进行循环
    for n in a.iter() {
      print!("\t{} + 10 = {}", n, n+10);
    }

    let mut sum = 0;
    // 0..a.len,是一个 Rust 的语法糖，其实就等于一个数组，元素是从0,1,2一直增加到到a.len-1
    for i in 0..a.len() {
      sum += a[i];
    }
    println!("\t({:?} = {})", a, sum);
  }
}
```

Rust-Course 的总结：

- 数组类型容易跟数组切片混淆，`[T;n]` 描述了一个数组的类型，而 `[T]` 描述了切片的类型， 因为切片是运行期的数据结构，它的长度无法在编译期得知，因此不能用 `[T;n]` 的形式去描述
- `[u8; 3]` 和 `[u8; 4]` 是不同的类型，数组的长度也是类型的一部分
- 在实际开发中，使用最多的是数组切片 `[T]` ，我们往往通过引用的方式去使用 `&[T]` ，因为后者有固定的类型大小

## Control

- `if` 语句块是表达式
- 用 `if` 来赋值时，要保证每个分支返回的类型一样（或结合 for continue）

循环

- `for _ in 0..=10` 到10，执行11次。
- 使用 `for` 时往往使用集合的引用形式，否则所有权将会被转移，除非是实现了 `copy trait` 的数组，如 `[i32;10]`。
- 如果想在循环中修改元素，使用 `mut` 关键字：
- 直接循环集合中的元素比用下标索引循环的 `for` 性能更优，因为是连续访问且不用边界检查
- `loop` = 无限循环的 `while`

```rust
let mut counter = 0;
let result = loop { // loop 是一个表达式
    counter += 1;
    if counter == 10 {
        break counter * 2; // 类似 return，赋值给 result
    }
};
println!("The result is {}", result);
```

[表达式使用](https://course.rs/appendix/expressions.html#if%E8%A1%A8%E8%BE%BE%E5%BC%8F)

```rust
let mut v = 0;
for i in 1..10 {
    v = if i == 9 {
        continue
    } else {
        i
    }
}
println!("{}", v); //8
```

## Pattern Match



## Lifetime

可以解决悬垂引用 (Dangling References) 的问题

```rust
fn main() {
    let reference_to_nothing = dangle();
}

// error: expected named lifetime parameter
fn dangle() -> &String { // dangle 返回一个字符串的引用
    let s = String::from("hello"); // s 是一个新字符串
    &s // 返回字符串 s 的引用
} // 这里 s 离开作用域并被丢弃。其内存被释放。

fn no_dangle() -> String {
    let s = String::from("hello");
    s
} // String 的所有权被转移给外面的调用者。
```

## 编译器属性标记

`#![...]` 将对整个文件有效, `#[...]` 只对该行下面的块有效
