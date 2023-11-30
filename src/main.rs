fn main() {
    let a: &'static str = "Hello, world!";

    let b: &'static str = r#"
        원시 문자열
    "#;

    println!("{ }", a);
    println!("{ }", b)
}
