fn main1() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);
}

fn deepCopy() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

}

fn shallowCopy() {
    let x: &str = "hello, world";
    let y = x;
    println!("{},{}",x,y);
}

fn main() {
    deepCopy();
}
