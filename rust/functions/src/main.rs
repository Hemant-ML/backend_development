fn main() {
    let z: i32 = my_function(10);
    println!("z is: {}",z)
}

fn my_function(x: i32) -> i32 {
    println!("x is: {x}");
    let y: i32 = x * 2;
    return y;
}