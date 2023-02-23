use num::complex::Complex;

struct Struct {
    e: i32
}

fn add(x:i32, y:i32) -> i32{
    x + y
}

fn main() {

    let mut _x = 30_i32;
    let y = 3;
    println!("{}", add(_x, y));

    
    let (a, mut b): (bool,bool) = (true, false);
    // a = true,不可变; b = false，可变
    println!("a = {:?}, b = {:?}", a, b);
    b = true;
    assert_eq!(a, b);


    let (a, b, c, d, e);
    (a, b) = (1, 2);
    // _ 代表匹配一个值，但是我们不关心具体的值是什么，因此没有使用一个变量名而是使用了 _
    [c, .., d, _] = [1, 2, 3, 4, 5];
    Struct { e, .. } = Struct { e: 5 };
    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);


    let x = 5;
    // 在main函数的作用域内对之前的x进行遮蔽
    let x = x + 1;
    {
        // 在当前的花括号作用域内，对之前的x进行遮蔽
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);
    
    // 字符串类型
    let spaces = "   ";
    // usize数值类型
    let spaces = spaces.len(); //同名变量遮蔽（shadowing 内存对象再分配）


    let y = 0.1;
    // let z = x + y; // no implementation for intergert + float
    println!("adding result: {}", y + x as f64);

    for i in 'a'..='d' {
        println!("{}", i);
    }


    let a = Complex {re: 2.1, im: -1.2};
    let b = Complex::new(11.1, 22.2);
    let result = a + b;

    println!("{} + {}", result.re, result.im);

}
