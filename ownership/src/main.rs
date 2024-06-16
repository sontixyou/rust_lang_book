fn main() {
    // ここでmoveメソッドが呼ばれるため、s1のownerではなくなる
    let s1 = String::from("hello");
    // let s2 = s1;
    // println!("{s1}, world!");

    // Stack only dataだから、コピーできる
    // データサイズが既知である場合、コンパイル時にコピーが行われる
    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}");
}
