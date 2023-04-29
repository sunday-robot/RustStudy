#[allow(dead_code)]
// 別ファイルから参照できるようにする場合、"pub"を付ける。
pub fn first() {
    let a = 10; // 型推論
    let b: i32 = 20; // 型を明示
    let c = 30i32; // 定数に型を明示
    let d = 40_i32; // アンダースコアで見やすく型を明示
    let e = add(add(a, b), add(c, d));

    // "println!"は、関数ではなくマクロで、マクロは関数を返すものであるとのこと
    println!("(a + b) + (c + d) = {}", e);

    let f = 10.0;
    let g: f64 = 20.0;
    let h = 30f64;
    let i = 40_f32;
    let j = add_f(add_f(f, g), add_f(h, i.into()));
    println!("(f + g) + (h + i) = {}", j);
}

// 関数定義
// 戻り値の型は"->i32"の様に明示しなければならないらしい。
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn add_f(a: f64, b: f64) -> f64 {
    a + b
}
// ↓Rustでは関数のオーバーロードは基本的にできない。(何やら小技を使うことでそれっぽいことができるらしいが、期待したものではない。)
/*
trait Overload<T> {
    fn add2(T)->T;
}

impl Overload<(i32, i32)> {
    fn add2(args:(i32, i32))->i32 {
    args.0+args.1
    }
}
*/
