#[allow(dead_code)]
pub fn reference_sample1() {
    let a = 12;
    let b = &a;
    let a = 23;
    let c  = a + b;
    println!("a={}, b={}, c={}", a, b, c);
}

#[allow(dead_code)]
pub fn reference_sample2() {
    let a = "12";
    let b = &a;
    let a = "23";
    let c  = [a, *b].concat();
    println!("a={}, b={}, c={}", a, b, c);
}
