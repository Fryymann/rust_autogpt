#[warn(unused_variables)]

// const OUR_COURSE: &str = "Rust with AutoGPT";

fn main() {

    // println!("Welcome to this curse on {}", OUR_COURSE);

    // // Stack
    // let x: i32;
    // x = 2;
    // println!("x is {}", x);

    // let y:i32 = 4;
    // println!("y is {}", y);

    // // for loop

    // for i in 0..y {
    //     if i != 4 {
    //         print!("{},  ", i);
    //     } else {
    //         println!("{} ", i);
    //     }
    // }

    // // Mutation

    // let mut z: i32 = 5;
    // print!("z was {} ", z);
    // z = 10;

    // let mut x: i32 = 10;
    // print!("x is {}", x);

    // let y: &mut i32 = &mut x;
    // *y = 15;
    // print!("x is {}", x);


    // let freezing_temp: f64 = -2.4;
    // print!("freezing temp is {}", freezing_temp);

    // let is_zero_remainder: bool = 10 % 4 != 0;
    // println!("is_zero_remainder is {}", is_zero_remainder);

    // let my_char: char = 'z';
    // println!("my_char is {}", my_char);


    // let emoji_char: char =  '😎';

    // println!("emoji_char is {}", emoji_char);

    // let my_floats: [f32; 10] = [0.0; 10];
    // println!("my_floats are {:?}", my_floats);

    // let my_floats_new: [f32; 10] = my_floats.map(|n:f32| n + 2.0);
    // println!("my_floats_new are {:?}", my_floats_new);

    // let test_arr: Vec<i32> = vec![2 ;3];
    // dbg!(&test_arr);

    // println!("Big Number is {}", 98_222_000);
    // println!("Hex is {}", 0xff);
    // println!("Octal is {}", 0o77);
    // println!("Binary is {}", 0b1111_0000);
    // println!("Bytes 'A' is {}", b'A');

    // // Raw - String Literal
    // let text: &str =r#"{"message" : "Rust is awesome"}"#;
    // dbg!(text);

    let a: u8 = 0b_1010_1010;

    let b: u8 = 0b_0101_1010;
    println!("a's value is {}", a);
    // println!("b's value is {}", b);

    println!("a's in binary {:08b}", a);
    // println!("b's in binary {:08b}", b);

    // // Logic Gates
    // println!("AND {:08b}", a & b);
    // println!("OR {:08b}", a | b);
    // println!("XOR {:08b}", a ^ b);
    // println!("NOT {:08b}", !a);

    // Bitwise Operations
    println!("a << 1 {:08b}", a << 1);




}
