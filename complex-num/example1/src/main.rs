

struct Complex {
    re: f64,
    im: f64, 
}

fn main() {
    let mut a: Complex; // a绑定了一个Complex
    a = Complex { re: 2.1, im: -1.2}; // 修改a的值
    // 上面的代码可以类比下面, 就是处理的数据类型不同罢了
    let mut age: u8; // age绑定了u8
    age = 18; // 修改age的值
}