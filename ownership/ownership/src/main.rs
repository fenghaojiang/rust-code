fn main1() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);
}


fn main() {
    let x: &str = "hello, world";
    let y = x;
    println!("{},{}",x,y);
}
