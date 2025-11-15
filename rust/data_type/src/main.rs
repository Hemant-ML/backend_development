const MAX_POINTS: u32 = 100_000;
static COUNTER: u32 = 0;

fn main() {
    //boolean
    let b1: bool = true;

    //unsigned integer
    let u1: u8 = 255;

    //signed integer
    let i1: i8 = -128;

    //floating point
    let f1: f32 = 3.14;

    //character, &str, and string
    let c1: char = 'a';
    let s1: &str = "hello";
    let s2: String = String::from("hello");
    
    //array
    let a1: [i32; 3] = [1, 2, 3];

    //tuple
    let t1: (i32, f32, char) = (1, 3.14, 'a');

    //type alias
    type MyInt = i32;
    let my_int: MyInt = 10;

    println!("my_int is: {my_int}");

    let c: u32 = COUNTER ;
    println!("c is: {c}");
}
