// 素直な実装だが、タプルやstructを使うことで、可読性を上げることができる
// fn main() {
//     let width1 = 30;
//     let height1 = 50;
//
//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(width1, height1)
//     );
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// tupleの要素が、.0, .1というようにアクセスできるが、これが何を指しているのかわからないため、バグの原因になる
// fn main() {
//     let rect1 = (30, 50);
//
//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect1)
//     );
// }
//
// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// this is better code than above.
// struct Rectangle {
//     width: u32,
//     height: u32,
// }
//
// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };
//
//     // println is formatting macro
//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(&rect1)
//     );
// }

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// rect:? is displying debug information. this statement is needed to load the Debug trait.
// rect:#? is pretty debug print.
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {rect1:#?}");
}
