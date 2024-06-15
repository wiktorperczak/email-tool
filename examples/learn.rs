fn test(s1 : String) -> String {
    println!("{}", s1);
    s1
}

fn test1(s1 : &String) {
    println!("Test1: {}", s1);
}

fn main() {
    let s1 = String::from("Test string");
    // let s2 = test(s1);
    // println!("{}", s2);

    let s2 = &s1;
    test1(s2);
    println!("{}", s1);
}