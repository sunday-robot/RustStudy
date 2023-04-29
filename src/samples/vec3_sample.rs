use crate::types::Vec3;

#[allow(dead_code)]
pub fn vec3_sample() {
    let mut a = Vec3::new(1.0, 2.0, 3.0);
    let b = Vec3::new(2.0, 3.0, 4.0);
    let c = a + b;
    println!("a:{} + b:{} = c:{}", a, b, c);
    a += b;
    println!("a += b ->{}", a);
    a -= b;
    println!("a -= b ->{}", a);
    println!("a.squared_length() = {}", a.squared_length());
    println!("a.length() = {}", a.length());
}
