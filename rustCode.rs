//use std::io;

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    println!("Hello, world!");
    
    //unmutable variable
    let x1 = 0.56;
    //mutable variable (mut)
    let mut x2 = 1;
    
    println!("unmutable varaible: {}", x1);
    x2 = 4;
    println!("mutable variable: {}", x2);
    
    //shadowing variables
    let x3 = 2;
    let x3 = x3 + 5;
    let x3 = x3 * 4;
    //this way it will prevent you from doing x3 += 3 or *= 7
    println!("shadowed variable: {}", x3);
    
    
    let spaces1 = "        ";
    let spaces1 = spaces1.len();
    
    let mut spaces2 = "   ";
    spaces2 = "       ";
    //not spaces2 = spaces2.len()
    let spaces2_length = spaces2.len();
    
    //explicit type declaration
    let mut my_bool: bool = false;
    
    //rust allows using more than ascii
    let heart_eyed_cat = '😻';
    
    let my_tuple = (500, 6.4, 1);
    let (x, y, z) = my_tuple;
    println!("The value of y is: {}", y);
    println!("The 0th value of my_tuple is: {}", my_tuple.0);
    
    let a = [1, 2, 3, 4, 5];
    let index = 3;

    let element = a[index];
    
    println!("{}",element);
    
    let var_a: i32 = 5;
    let var_b: i32 = 15;
    let result = add(var_a, var_b);
    println!("{} plus {} = {}", var_a, var_b, result);
    
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }
    for number in (1..5).rev() {
        println!("{}", number);
    }
    
    loop
    {
        if my_bool {break;}
        else {x2 = add(x2, 1); if x2 > 6 {my_bool = true;}}
        println!("running loop...");
    }
    
    struct vec3(i32, i32, i32);
    let normal = vec3(0,1,0);
    
    struct colour {
        R: i32,
        G: i32,
        B: i32,
    }
    let red = colour{
        R: 1,
        G: 0,
        B: 0,
    };
    
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}


fn add(a: i32, b: i32) -> i32
{
    a + b
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}