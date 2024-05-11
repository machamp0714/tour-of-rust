fn main() {
    println!("-------if文-------");
    let number = 5;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // if number { expected `bool`, found integer
    //     println!("number was five");
    // }

    let condition = true;
    let number = if condition { 5 } else { 6 }; // let文内で式を使う

    println!("The value of number is: {}", number);
    println!("-------if文-------\n");

    println!("-------ループ-------");
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
    println!("-------ループ-------\n");

    println!("-------while文-------");
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");
    println!("-------while文-------\n");

    println!("-------for文-------");
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    for element in a {
        println!("the value is: {}", element);
    }
    println!("-------for文-------\n");
}
