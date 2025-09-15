// std::fmt::Display;
use std::io;

fn main() {
    let sum = 5 + 10;
    println!("sum: {}", sum);

    let diff = 95.5 - 4.3;
    println!("diff: {}", diff);

    let prod = 4 * 30;
    println!("prod: {}", prod);

    let quotient = 56.7 / 32.2;
    let floored = 2 / 3;
    let remainder = 43 % 5;
    println!("quotient: {}, floored: {}, remainder: {}", 
            quotient, floored, remainder);

    let t = true;
    println!("t: {}", t);

    let f: bool = false;
    println!("f: {}", f);

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("c: {}, z: {}, heart_eyed_cat: {}", c, z, heart_eyed_cat);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("{}, {}, {}", five_hundred, six_point_four, one);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let months = ["January", "Feburary", "March", "April", "May", "June", "July", 
                                    "August", "September", "October", "November", "December"];
    //let a = [3; 5]; // [3, 3, 3, 3, 3]と初期化
    let first = a[0];
    let second = a[1];

    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}", 
        index, element
    );

    another_function(5);
    print_labeled_measurement(5, 'h');


    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    let mut count = 0;
    'couting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            } 
            if count == 2 {
                break 'couting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);

    let mut num = 3;
    while num != 0 {
        println!("{}!", num);

        num -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20 ,30, 40, 50];
    // let mut index = 0;
    // while index < 5 {
    //     println!("the value is: {}", a[index]);
    //     index += 1;
    // }
    for elem in a {
        println!("the value is: {}", elem);
    }

    for n in (1..4).rev() {
        println!("{}!", n);
    }
    println!("LIFTOFF!!!");

}

fn another_function(x: i32) { // 型宣言は必須
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}
