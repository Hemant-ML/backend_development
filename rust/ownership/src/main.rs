fn main() {
    
    let s1:String = String::from("Rust");

    // take_ownership(s1);
    take_ownership_clone(s1.clone());
    println!("s1: {}", s1);
    println!("s1: {}", s1); // s1 is not valid here, because it was moved to the function
    let s2:String = add_to_string(s1);
    println!("s2: {}", s2);



    // let s2:String = s1.clone();
    // println!("s1: {}", s1);
    // println!("s2: {}", s2);
    
    /*  for primitive datatype default behavior, the value is copied into the new variable, 
    so the original value is not moved to the new variable. */
    let x:i32 = 5;
    let y:i32 = x;
    println!("x: {}", x);
    println!("y: {}", y);
}

fn take_ownership(s:String) {
    println!("s: {}", s);
}

fn take_ownership_clone(s:String) {
    println!("s: {}", s);
}

fn add_to_string(mut s:String) -> String {
    s.push_str(" is awesome");
    s
}