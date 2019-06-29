/*
 * Rustのライフタイム省略規則。
 * CreatedAt: 2019-06-29
 */
fn main() {
    println!("{}", longest("AA", "A"));
}
//fn longest(x: &str, y: &str) -> &str { // error[E0106]: missing lifetime specifier
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x }
    else { y }
}

