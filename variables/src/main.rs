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
}
