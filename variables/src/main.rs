fn main() {
    println!("-------文字列--------");
    let c = 'z';
    print_typename(c);

    let d = "Z";
    print_typename(d);

    let e = "eeeeee";
    print_typename(e);
    println!("-------文字列--------");

    println!("-------複合型--------");
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup; // 分配
    println!("x: {} | y: {} | z: {}", x, y, z);
    println!("tup.0: {}", tup.0);

    let arr: [i32; 5] = [1, 2, 3, 4, 5]; // [データ型; 要素数]
    let arr = [3; 5]; // [3, 3, 3, 3, 3]
                      // println!("arr: {}", arr[5]); index out of bounds: the length is 5 but the index is 5
    println!("-------複合型--------");
}

fn print_typename<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}
