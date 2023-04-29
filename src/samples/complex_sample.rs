//! 標準ライブラリの複素数クラスのサンプル
use num::Complex;

#[allow(dead_code)]
pub fn complex_sample() {
    // Rustにおける通常のインスタンス生成
    let a = Complex {
        re: 2.1,
        im: -1.0 / 2.0,
    };

    // newという名前のメソッドでインスタンスを生成。
    // newという名のメソッドを設けることは慣習に過ぎず、Rustの言語仕様というわけではない。
    let b = Complex::new(11.1, 22.2);

    let result = a + b;
    println!("{} + {} = {}", a, b, result);
}
