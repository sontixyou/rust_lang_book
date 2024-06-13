fn main() {
    // Statement なにかしらのアクションをするだけで、値を返さない
    let x = 5;

    // Expression 値を返す
    5 + 6;
    // 関数やマクロの呼び出しもexpression
    // 中括弧で囲まれたブロックもexpression。だが、末尾にセミコロンを付けるとstatementになる
    {
        let x = 3;
        x + 1
    }
    println!("Hello, world!");
}
