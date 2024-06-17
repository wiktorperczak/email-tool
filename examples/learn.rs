fn test(s : String) -> String {
    println!("{}", s);
    s
}

fn test1(s : &mut String) {
    *s = s.to_lowercase();
    println!("{}", s);
}

fn main() {
    let mut s = String::from("HELLO");
    s = test(s);
    println!("New: {}", s);
    test1(&mut s);
}