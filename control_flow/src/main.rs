fn main() {
    let number = 3;

    // ifの条件式はbool型でなければならない
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    // これはarmがどちらもIntegerであるため、OK
    let number = if number < 5 { 5 } else { 6 };
    // これはarmが異なる型であるため、エラー
    // let missing_number = if number < 5 { 5 } else { "six" };
    //
    //
    // repetition with loops
    loop {
        println!("again!");
        // 自分でloopを止めるコードを書かない限り、永遠にループし続ける
        break;
    }
    // 多重loopの各loopにラベルを付けることができる
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                // このbreakはcounting_up loopを抜ける
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // for
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
