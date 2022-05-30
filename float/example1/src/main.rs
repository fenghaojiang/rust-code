fn main() {
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("   0.1 + 0.2 : {:x}", (abc.0 + abc.1).to_bits());
    println!("         0.3 : {:x}", (abc.2).to_bits());

    println!();

    println!("xyz (f64)");
    println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("          0.3: {:x}", (xyz.2).to_bits());

    println!();

    // assert!(abc.0 + abc.1 == abc.2);
    // assert!(xyz.0 + xyz.1 == xyz.2);

    let twenty = 20;
    let twenty_one:i32 = 21;
    let twenty_two = 22i32;

    let addition = twenty + twenty_one + twenty_two;

    println!("{} + {} + {}  = {}", twenty, twenty_one, twenty_two, addition);



    let one_million:i64 = 1_000_000;
    println!("{}", one_million.pow(2));

    let forty_two = [
        42.0,
        42f32,
        42.0_f32,
    ];

    println!("{:.2}", forty_two[0]);

    let a:i32 = 2;
    // 二进制为00000011
    let b:i32 = 3;

    println!("(a & b) value is {}", a & b);

    println!("(a | b) value is {}", a | b);

    println!("(a ^ b) value is {}", a ^ b);

    println!("(!b) value is {} ", !b);

    println!("(a << b) value is {}", a << b);

    println!("(a >> b) value is {}", a >> b);

    let mut a = a;
    // 注意这些计算符除了!之外都可以加上=进行赋值 (因为!=要用来判断不等于)
    a <<= b;
    println!("(a << b) value is {}", a);


     for i in 1..=5 {
         println!("{}", i);
     }

}
