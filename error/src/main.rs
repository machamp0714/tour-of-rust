use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    // let v = vec![1, 2, 3];

    // v[99];

    //  let f = File::open("hello.txt");
    // let f = File::open("hello.txt").unwrap(); // unwrap は Result 型の値が Ok の場合は Ok の中身を返し、Err の場合は panic! マクロを呼び出す
    // let f = File::open("hello.txt").expect("Failed to open hello.txt"); // expect は unwrap と同じだが、エラーメッセージを指定できる

    // let f = match f {
    //     Ok(file) => file,
    //     Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("hello.txt") {
    //         // マッチガードという機能を使って、エラーが NotFound の場合のみ処理を行うようにしている
    //         // ref はガード条件式に error がムーブされないように必要
    //         Ok(fc) => fc,
    //         Err(e) => {
    //             panic!("Tried to create file but there was a problem: {:?}", e)
    //         }
    //     },

    //     Err(error) => {
    //         panic!("There was a problem opening the file: {:?}", error)
    //     }
    // };

    let str = read_username_from_file();
    match str {
        Ok(s) => println!("{}", s),
        Err(e) => println!("{:?}", e),
    }
}

// エラーを委譲する
fn read_username_from_file() -> Result<String, io::Error> {
    // let f = File::open("hello.txt");

    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    // let mut s = String::new();

    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }

    // ?演算子はResultの値がOkならOkの中身を返し、処理を継続する。ErrならreturnをしたかのようにErrの中身を返す
    // let mut f = File::open("hello.txt")?;
    // let mut s = String::new();
    // f.read_to_string(&mut s)?;
    // Ok(s)

    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
