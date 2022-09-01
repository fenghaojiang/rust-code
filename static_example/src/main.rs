use std::fmt::Display;


fn main() {
    let r1;
    let r2;
    {
        static STATIC_EXAMPLE:i32 = 42;
        r1 = &STATIC_EXAMPLE;
        let x = "&'static str";
        r2 = x;
    }
    let r3: &str;
    {
        let s1 = "String".to_string();
        static_bound(&s1);
        r3 = &s1;

    }   
    println!("{}", r3); 
}

fn static_bound<T: Display + 'static> (t: &T) {
    println!("{}", t);
}
