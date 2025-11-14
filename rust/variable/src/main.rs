fn main() {
    // crearion
    let a: i8 = 5;

    //mutability
    let mut b: i8 = 6;
    b = 7;
    // shadowing
    let c: i32 = 10;
    let c: i32 = 20;

    //scope
    let d: i32 = 30;
    {
        let d: i32 = 40;
        println!("inner d is: {d}");
    }
    println!("a is : {a}");
    println!("b is: {b}");
    println!("c is: {c}");
    println!("outer d is: {d}");
}
