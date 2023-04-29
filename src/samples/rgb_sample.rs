use crate::types::Rgb;

#[allow(dead_code)]
pub fn rgb_sample() {
    let a = Rgb::new(1.0, 2.0, 3.0);
    let b = Rgb::new(2.0, 3.0, 4.0);
    let d = 0.25;
    println!("a:{} + b:{} -> {}", a, b, a + b);
    println!("a:{} * b:{} -> {}", a, b, a * b);
    println!("a:{} * d:{} -> {}", a, d, a * d);
    println!("a:{} / d:{} -> {}", a, d, a / d);
}
