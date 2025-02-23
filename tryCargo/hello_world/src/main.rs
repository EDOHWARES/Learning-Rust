// fn main() {
//     // Tuples
//     let tup: (i32, f64, u8) = (500, 6.4, 1);

//     // Destructuring
//     let (_x, y, _z) = tup;
//     println!("The value of y is: {}", y);

//     // Access by index
//     println!("{}", tup.0);
// }

// fn main() {
//     // ARRAYS
//     let arr = [1, 2, 3, 4, 5];

//     let first = arr[0];
//     let second = arr[1];

//     println!("first: {}, second: {}", first, second);

//     for element in arr.iter() {
//         println!("Elemetnt{}, {}", element, element+1);
//     }
// }

// fn main() {
//     // CUSTOM DATA TYPES
//         // STRUCTS
//     struct Person {
//         name: String,
//         age: u8,
//     }

//     struct Animal {
//         name: String,
//         number_of_legs: u8,
//         breed: (String, String, String),
//         age: f32,
//         male: bool,
//     }

//     let person = Person {
//         name: String::from("Edoh"),
//         age: 20,
//     };

//     let my_pet = Animal {
//         name: String::from("Bingo"),
//         number_of_legs: 4,
//         breed: (String::from("Lassa"), String::from("German Shepherd"), String::from("Chaw-Chaw")),
//         age: 3.0,
//         male: false,
//     };

//     println!("His name is {} and age is {}", person.name, person.age);
//     println!("{} has {} legs and is {}years old, and is a {}. It's a cross breed of: {}, {}, {}",
//     my_pet.name,
//     my_pet.number_of_legs,
//     my_pet.age,
//     if my_pet.male
//     {
//         "Male"
//     } else {
//         "Female"
//     },
//     my_pet.breed.0,
//     my_pet.breed.1,
//     my_pet.breed.2,
// )

// }

// fn main() {
//     // CUSTOM DATA TYPES
//         // ENUMS

//     enum TrafficLight {
//         Red, Yellow, Green,
//     }

//     let light = TrafficLight::Red;
//     println!("Traffic light color: {}", light)

// }

// use std::process::Command;

// fn main() {
//     // Suspend the system using systemctl
//     let status = Command::new("systemctl")
//         .arg("suspend")
//         .status()
//         .expect("Failed to execute suspend command");

//     println!("Command exited with: {}", status);
// }

// use std::io;

// fn main() {
//     let arr = [1, 2, 3, 4, 5];

//     println!("Please enter an array index:");

//     let mut index = String::new();

//     io::stdin()
//         .read_line(&mut index)
//         .expect("Failed to read line");

//     let index: usize = index
//         .trim()
//         .parse()
//         .expect("The index entered was not a number");

//     let element = arr[index];

//     println!("The value of the element at index {} is: {}", index, element);
// }

// use std::io;

// fn main() {
//     let countries: [&str; 10] = [
//         "Nigeria",
//         "Ghana",
//         "Mexico",
//         "United States",
//         "United Kingdom",
//         "Jamaica",
//         "Cameroon",
//         "Niger",
//         "Botswana",
//         "China",
//     ];

//     let mut index = String::new();
//     println!("Enter an index (0-9) to get a city name: ");

//     io::stdin()
//         .read_line(&mut index)
//         .expect("Unable to read line!!!");

//     let index: usize = index
//         .trim()
//         .parse()
//         .expect("Entered value was not a number");

//     println!("The city at index {index} is: {}", countries[index]);
// }

// fn main() {
//     println!("Main function.");
//     another_function(44, 'x');
// }

// fn another_function(num: usize, letter: char) {
//     println!("The value of num is: {num}");
//     println!("The value of letter is: {letter}");
// }

// fn main() {

//     let x = sum_diff(1, 2);
//     // println!("The sum is: {} and the difference is: {}", x.0, x.1);
//     println!("The sum and difference are {:?}", x);

// }

// fn sum_diff(num1: i32, num2: i32) -> (i32, i32) {
//     return (num1 + num2, num1 - num2)
// }

// BASIC CONTROL FLOW IN RUST
// If expressions
// Match
// Loop expressions
// While expressions
// For expressions

// fn main() {
//    let condition = true;

//    let number = if condition {
//     5
//    } else {
//     6
//    };

//    println!("The value of number is {number}")
// }

// Nested if expressions
// fn main() {
//     let num = 9;

//     if num % 2 == 0 {
//         println!("{} is even", num)
//     } else {
//         println!("{} is odd", num);

//         if num > 10 {
//             println!("{} is also greater than 10", num);
//         } else {
//             println!("{} is not greater than 10", num);
//         }
//     }
//  }

// using && and || operators
// fn main() {
//     let a = 10;
//     let b = 5;
//     let c = 4;

//     if a > b && b > c {
//         println!("a is greater than be and b is greater than c");
//     } else {
//         println!("condition with && is not met");
//     };

//     if a > b || b > c {
//         println!("At least one condition with || is met");
//     } else {
//         println!{"The condition with || is not met"};
//     };
//  }

// fn main() {
//     enum Coin {
//         Penny,
//         Nickel,
//         Dime,
//         Quarter,
//     }

//     fn value_inc_cents(coin: Coin) -> u8 {
//         match coin {
//             Coin::Penny => 1,
//             Coin::Nickel => 5,
//             Coin::Dime => 10,
//             Coin::Quarter => 25,
//         }
//     }

//     let coin = Coin::Penny;

//     println!("Value of coin: {}", value_inc_cents(coin));
//  }

// fn main() {
//     enum TrafficLight {
//         Red,
//         Yellow,
//         Green,
//     }

//     let light = TrafficLight::Red;

//     fn func(light: TrafficLight) -> String {
//         match light {
//             TrafficLight::Red => String::from("Stop"),
//             TrafficLight::Yellow => String::from("Slow down"),
//             TrafficLight::Green => String::from("Go"),
//         }
//     }

//     println!("The light is: {}", func(light));
//  }

// Loop
//  fn main() {
//     let mut counter = 0;

//     let result = loop {
//         println!("counter is {} now", counter);
//         counter += 1;

//         if counter == 10 {
//             break counter * 2;
//         }
//     };

//     println!{"The result is: {}", result};
//  }

// While loop
// fn main() {
//     let mut count = 20;

//     while count != 0 {
//         println!("{}!", count);

//         count -= 1;

//         //wait for 1 second
//         std::thread::sleep(std::time::Duration::from_secs(1));
//     };
//  }

// For loop
// fn main() {
//     let a = [1, 2, 3, 4, 5];

//     for element in a.iter() {
//         println!("The value is: {}", element);
//     }

//     let s = "Hello world!";

//     for element in s.chars() {
//         println!("The value is: {}", element);
//     }

//     for number in 1..10 {
//         println!("The number is: {}", number);
//     }
//  }

// fn main() {
//     for i in 1..101 {
//         if i%3 == 0 && i%5 == 0 {
//             println!("FizzBuzz");
//         } else if i%3 == 0 {
//             println!("Fizz");
//         } else if i%5 == 0 {
//             println!("Buzz");
//         } else {
//             println!("{}", i);
//         }
//     }
// }

// fn main() {
//     for i in 1..101 {
//         if i % 3 == 0 {
//             if i % 5 == 0 {
//                 println!("FizzBuzz");
//             } else {
//                 println!("Fizz");
//             }
//         } else if i % 5 == 0 {
//             println!("Buzz");
//         } else {
//             println!("{}", i);
//         }
//     }
// }


// fn main() {
//     let mut s = String::from("Hello");

//     s.push_str(", world!");

//     println!("{}", s);
// }

fn main() {
   let s1 = String::from("Hello world");
   let s2 = s1.clone();

   println!("{s2}");
   println!("{s1}");
}