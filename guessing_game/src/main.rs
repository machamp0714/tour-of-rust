// Rustは標準ライブラリで定義されているいくつかをスコープに取り込み、これをプレリュードと呼ぶ。
// プレリュードにない場合は、useで明示的にスコープに取り込む必要がある。
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new(); // 変数を可変(mutable)にするためには mut を付ける

    io::stdin() // use を使わなくても、std::io::stdin() として呼び出すこともできる
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
