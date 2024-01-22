// fn main() {
//     let number = 2cl;

//     if number % 4 == 0 {
//         println! ("number is divisible by 4");
//     } else if number % 3 == 0 {
//         println! ("number is divisible by 3");
//     } else {
//         println! ("number is not divisible by 4, 3")
//     }
// }


// fn main() {
//     let condition = true;
//     let number = if condition { 5 } else { 6 };

//     println!("The value of number is: {number}");
// }


// fn main() {
//     loop {
//         println!("again!");
//     }
// }


// fn main() {
//     let mut counter = 0;

//     let result = loop {
//         counter += 1;

//         if counter == 10 {
//             break counter * 2;
//         }
//     };

//     println!("The result is {result}");
// }   


// fn main() {
//     let mut count = 0;
//     'counting_up: loop {
//         println!("count = {count}");
//         let mut remaining = 10;
    
//         loop {
//             println!("remaining = {remaining}");
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }

//         count += 1;
//     }
//     println! ("End count = {count}");
// }


// fn main() {
//     let mut number = 3;

//     while number != 0 {
//         println!("{number}!");

//         number -= 1;
//     }
    
//     println!("LIFTOFF!!!");
// }


// fn main() {
//     let a = [10, 20, 30, 40, 50];
//     let mut index = 0;

//     while index < 5 {
//         println!("the value is: {}", a[index]);

//         index += 1;
//     }
// }


// fn main() {
//     let a = [10, 20, 30, 40, 50];

//     for element in a {
//         println!("the value is {element}");
//     }
// }


// fn main() {
//     for number in (1..4).rev() {
//         println!("{number}"!);
//     }
//     println!("LIFTOFF");
// }


// fn fahrenheit_to_celsius(f: f64) -> f64 {
//     (f - 32.0) * 5.0 / 9.0
// }

// fn celsius_to_fahrenheit(c: f64) -> f64 {
//     c * 9.0 / 5.0 + 32.0
// }

// fn main() {
//     let f_temp = 32.0;
//     let c_temp = fahrenheit_to_celsius(f_temp);
//     println!("{} Fahrenheit is {} Celsius", f_temp, c_temp);

//     let c_temp = 0.0;
//     let f_temp = celsius_to_fahrenheit(c_temp);
//     println!("{} Celsius is {} Fahrenheit", c_temp, f_temp);
// }


// fn fibonacci(n: u32) -> u32 {
//     match n {
//         0 => 0,
//         1 => 1,
//         _ => fibonacci(n - 1) + fibonacci(n - 2),
//     }
// }

// fn main() {
//     let n = 10;
//     let result = fibonacci(n);
//     println!("The {}th Fibonacci number is: {}", n, result);
// }


fn main() {
    let gifts = [
        "a partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    for day in 1..=12 {
        println!("On the {} day of Christmas, my true love sent to me:", ordinal(day));
        for gift_day in (1..=day).rev() {
            if gift_day == 1 && day != 1 {
                print!("And ");
            }
            println!("{}", gifts[(gift_day - 1) as usize]);
        }
        println!();
    }
}

fn ordinal(n: u32) -> String {
    let suffix = match n {
        1 => "st",
        2 => "nd",
        3 => "rd",
        _ => "th",
    };
    format!("{}{}", n, suffix)
}
