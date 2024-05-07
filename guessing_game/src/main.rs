// Rustは標準ライブラリで定義されているいくつかをスコープに取り込み、これをプレリュードと呼ぶ。
// プレリュードにない場合は、useで明示的にスコープに取り込む必要がある。
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // 変数を可変(mutable)にするためには mut を付ける

        io::stdin() // use を使わなくても、std::io::stdin() として呼び出すこともできる
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
