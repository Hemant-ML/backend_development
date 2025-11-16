fn main() {
    let mut s = String::from("Rust");
    let r1: &String = &s;
    print_string(r1);
    let r2=&mut s;
    add_to_string(r2);
    println!("main {}", s);
}

fn print_string(s1: &str) {
    println!("print_string {}", s1);
}

fn add_to_string(s1: &mut String) {
    s1.push_str(" is Awesome");
}
