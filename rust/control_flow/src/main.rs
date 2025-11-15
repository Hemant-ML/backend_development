fn main() {
    // if/else

    let a: i32 = 5;

    if a > 5{
        println!("bigger than 5");
    }
    else if a > 3{
    println!("bigger than 3");
    }
    else{
        println!("smaller or equal to 3");
    }

    let b:i32 = if a > 5 {10} else {20};
    println!("b is: {}",b);

    // loops

    let z:i8 = loop {
        break 5;
    };
    println!("z is: {}",z);

    //while loop
    let mut c:i32 = 0;
    while c < 10{
        println!("c is: {}",c);
        c += 1;
    }

    //for loop
    for i in 0..10{
        println!("i is: {}",i);
    }

    //for loop with range
    for i in (0..10).rev(){
        println!("i is: {}",i);
    }

    let array: [i32; 5] = [1, 2, 3, 4, 5];

    for element in array{
        println!("element is: {}",element);
    }
}
