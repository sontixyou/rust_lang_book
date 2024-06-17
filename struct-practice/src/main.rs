struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// struct tuple それぞれの要素の型を指定する
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    // mutableなので、値を変更できる
    // 特定のフィールドだけをミュータブルとしてマークすることはできません
    user1.email = String::from("anotheremail@example.com");

    // こうも書けるが、次のように書いた方が良い
    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };
    // ..構文を使うと、構造体の残りのフィールドを古い構造体からコピーできる。最後に書く必要がある。
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    // blackとoriginの各値は、それぞれ型が違う。
    // ただのi32ではない。originの値はPointというstruct tupleのi32である。
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}
