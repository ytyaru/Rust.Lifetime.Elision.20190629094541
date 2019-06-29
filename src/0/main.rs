/*
 * Rustのライフタイム省略規則。
 * CreatedAt: 2019-06-29
 */
fn main() {
    let first = first_word("Hello Rust !!");
    println!("{}", first);
}
fn first_word(s: &str) -> &str { // ライフタイム省略規則
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { return &s[0..i]; }
    }
    &s[..]
}
