fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // &s1 でs1の値を参照する参照を生成出来る

    println!("The length of '{}' is {}.", s1, len);

    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("{}", s2);

    // let reference_to_nothing = dangle();
}

fn calculate_length(s: &String) -> usize { // 関数の引数に参照を取ることを借用という
    s.len()
}

fn change(some_string: &mut String) { // 変数がデフォルト不変であるのと同様、参照も不変である
    some_string.push_str(", world");
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s // s の参照を返すが、s はスコープを抜けると解放されるため、この参照は無効になる。つまり危険。
}
