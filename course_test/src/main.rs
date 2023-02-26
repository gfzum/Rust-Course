fn main() {
    let mut s = String::from("hello"); // 必须 mut
    let p = &mut s;
    p.push_str("test"); // &str 没有 push_str 方法

    print!("{}", *p);

    let _a = [1,2,3];
    // let slice = a[1..3];

    // let s2 = s[1..3];
    // let s3 = &s2[1..2];
    // println!("{}", s3);
    let s = String::from("hello");
    str_test(&s);

    let t = (500, 5.2, 1);
    let (x, y, _) = t;
    let z = t.2;

    println!("{} {} {}", x, y, z) 
}

fn str_test(s : &str){
    print!("{}", s);
}
