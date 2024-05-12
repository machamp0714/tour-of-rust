fn main() {
    println!("------ムーブ------");
    let s1 = String::from("hello"); // ヒープに不明な量のメモリを確保する
    // let s2 = s1; // スタックにある、ポインタ、長さ、許容量のみがコピーされる。s1は s2 にムーブされた、と表現される
    let s2 = s1.clone(); // ヒープ領域のデータをコピーしている

    println!("s1: {}, s2: {}", s1, s2);
    println!("------ムーブ------\n");

    println!("------所有権と関数------");
    let s = String::from("hello"); // s がスコープに入る

    takes_ownership(s); // s が関数にムーブされる

    // println!("s: {}", s); value borrowed here after move

    let x = 5;

    makes_copy(x); // x も関数にムーブされるが、i32 は Copy なので、この後に x を使っても問題ない

    println!("x: {}", x);
    println!("------所有権と関数------\n");

    println!("------所有権と返り値------");
    let x1 = gives_ownership(); // gives_ownership が返す値が x1 にムーブされる

    let x2 = String::from("hello"); // x2 がスコープに入る

    let x3 = takes_and_gives_back(x2); // x2 が takes_and_gives_back にムーブされ、戻り値も x3 にムーブされる

    println!("x1: {}, x3: {}", x1, x3);
} // ここで x がスコープを抜け、s もスコープを抜ける。ただし、s はすでに take_ownership にムーブされているため、何も起こらない。

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}