fn main() {
    let mut s = String::from("hello"); // 必须 mut
    let p = &mut s;
    p.push_str("test"); // &str 没有 push_str 方法

    print!("{}", *p);
}
