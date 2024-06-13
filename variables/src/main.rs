fn main() {
    // これはShadowingではない
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // これはShadowing
    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y is: {}", y);
    }
    println!("The value of y is: {}", y);

    const MAX_POINTS: i32 = 100_000;

    println!("The value MAX_POINTS is: {}", MAX_POINTS);

    // Data type
    // 型を指定しないとエラーになる。型の推論が出来ないため。
    let guess: u32 = "42".parse().expect("Not a number!");

    // Taple
    // 一つのタプルを複数に分解することをdestructuringという
    // 各要素の型は異なっていても良い
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    // Array
    // 配列の要素は全て同じ型でなければならない
    // 要素数は固定である
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let a = [3; 5];
}
