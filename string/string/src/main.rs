fn main() {
    let mut string_truncate = String::from("Rust is awesome");
    string_truncate.truncate(10);
    dbg!(string_truncate);
}
