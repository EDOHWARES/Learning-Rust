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

// fn main() {
//    let s1 = String::from("Hello world");
//    let s2 = s1.clone();

//    println!("{s2}");
//    println!("{s1}");
// }

// fn main() {
//    let mut count = 0;

//    'counting_up: loop {
//         println!("count = {count}");
//         let mut  remaining = 10;

//         loop {
//              println!("remaining = {remaining}");
//              if remaining == 9 {
//                break;
//              }
//              if count == 2 {
//                break 'counting_up;
//              }
//              remaining -= 1;
//         }
//         count += 1;
//    }

//    println!("End count = {count}");
// }

// While loop
// fn main() {
//   let mut count = 3;

//   while count !=0 {
//       println!("{count}");

//       count -= 1;
//   }

//   println!("Lift off!!!");
// }

// // Loop Loop (Infinit Loop)
// fn main() {
//    let mut count = 3;
//    loop {
//       println!("{count}");

//       if count == 1 {
//          break;
//       }

//       count -= 1;
//    }

//    println!("Lift off!!!");
//  }

//  fn main() {
//    for number in (1..=3).rev() {
//       println!("{number}");
//    }
//    println!("Lift off!!!");
//  }

// fn main() {
//    let i = 5;
//    call_int(i);
//    println!("AFTER CALLING THE FUNCTION, the value of i: {}", i);

//    let s = String::from("Hello");
//    call_string(&s);
//    println!("AFTER CALLING THE FUNCTION, the value of s: {}", s);
// }

// // call int function
// fn call_int(i: i32) {
//    println!("call_int i: {}", i);
// }

// // call string function
// fn call_string(s: String) {
//    println!("call_string s: {}", s);
// }

// fn main() {
//    let s1 = give_ownership();
//    println!("s1: {}", s1);

//    let s2 = String::from("Hello from main");

//    let s3 = take_and_give_ownership(s2);
//    println!("s2: {}", s2);
//    println!("s3: {}", s3);
// }

// // function to give ownership of a string to another function
// fn give_ownership() -> String {
//    let some_string = String::from("Hello from give ownership");
//    some_string
// }

// // function to take ownership of a string
// fn take_and_give_ownership(some_string: String) -> String {
//    some_string
// }

// fn main() {
//    let s1 = String::from("Hello");

//    let len = calculate_length(&s1);

//    println!("The length of {} is {}", s1, len);
// }

// // calculate the length of a string
// fn calculate_length(s: &String) -> usize {
//    let length = s.len();
//    length
// }

// fn main() {
//    let mut s = String::from("Hello");
//    change_borrowed_value(&mut s);
//    println!("S: {}", s);
// }

// // fn to change borrowed value
// fn change_borrowed_value(s: &mut String) {
//    s.push_str(", world");
// }

// fn main() {
//    let mut s = String::from("Hello");

//    let s1 = &mut s;
//    let s2 = &mut s;

//    println!("{}", s1);
// }

// fn main() {
//    let mut s = String::from("Hello");

//    {
//       let s1 = &mut s;
//       s1.push_str(", world");
//    }

//    let s2 = &mut s;
//    s2.push_str("!!!");
//    println!("{}", s);
// }

// fn main() {

//    // first example: slice of an array of chars
//    let arr: [char; 5] = ['a', 'b', 'c', 'd', 'e'];
//    let slice: &[char] = &arr[1..3];
//    println!("{:?}", slice);

//    // second example: slice of a vector of integers
//    let vec: Vec<i32> = vec![10, 20, 30, 40, 50];
//    let slice: &[i32] = &vec[3..4];
//    println!("{:?}", slice);

//    // third example: slices for strings
//    let s: String = String::from("Hello world");
//    let hello: &str = &s[0..5];
//    let world: &str = &s[6..=10];
//    println!("{:?}", hello);
//    println!("{:?}", world);

// }

// fn main() {
//    let s = String::from("Francesco");

//    // shortcut for initial index
//    let slice = &s[0..3];
//    println!("{}", slice);
//    let slice = &s[..3];
//    println!("{}", slice);

//    // shortcut for final index
//    let len = s.len();
//    let slice: &str = &s[4..len];
//    println!("{}", slice);
//    let slice: &str = &s[4..];
//    println!("{}", slice);

//    // shortcut for both initial and final
//    let slice: &str = &s[..];
//    println!("{}", slice);
//    let slice: &str = &s[0..len];
//    println!("{}", slice);
// }

// fn main() {
//    let mut s = String::from("Hellooo World");
//    let word = first_word(&s);

//    println!("The s is = {}", s);
//    println!("The first word is = {} long", word);

//    // problem
//    s.clear();
//    println!("the s is = {}", s);
//    println!("the first word is = {}", word);
// }

// fn first_word(s: &String) -> usize {
//    let bytes = s.as_bytes(); // Convert string to bytes

//    // Iterate through the bytes and return the index of the first space
//    for (i, &item) in bytes.iter().enumerate() {
//       if item == b' ' {
//          return  i;
//       }
//    }

//    // if no space is found, return the length of the string
//    s.len()
// }

// Struct in RUST
// struct with name, email, is_active, age
// #[derive(Debug)]
// struct User {
//    name: String,
//    email: String,
//    is_active: bool,
//    age: u8
// }
// fn main() {
//    let user1 = User {
//       name: String::from("Edoh Emmanuel"),
//       email: String::from("edoh@gmail.com"),
//       is_active: false,
//       age: 20
//    };

//    // create a new instance from another instance
//    let user2 = User {
//       name: String::from("Ayomide Adeniran"),
//       email: user1.email.clone(),
//       is_active: user1.is_active,
//       age: user1.age
//    };

//    println!("{:?}", user1);
//    println!("{:?}", user2);

// }

// // create a function to build a user
// fn buid_user(name: String, email: String, is_active: bool, age: u8) -> User {
//    User {
//       name,
//       email,
//       is_active,
//       age
//    }
// }

//tuple structs
// struct  Color(i32, i32, i32);
// struct  Point(i32, i32, i32);

// fn  main() {
//    let black = Color(0, 0, 0);
//    let origin = Point(0, 0, 0);

//    println!("black: {}, {}, {}", black.0, black.1, black.2);
//    println!("origin: {}, {}, {}", origin.0, origin.1, origin.2);
// }

//unit-like struct
// #[derive(Debug)]
// struct User;

// fn main() {
//    let user1 = User;
//    print!("{:?} \n", user1)
// }

// fn main() {
//     let width: i32 = 50;
//     let height: i32 = 30;

//     println!("The area is {}", area(width, height));
// }

// fn area(w: i32, h: i32) -> i32 {
//    w * h
// }

// fn main() {
//    let rect1: (i32, i32) = (30, 50);

//    println!("The area is {}", area(rect1));
// }

// fn area(sides: (i32, i32)) -> i32 {
//    sides.0 * sides.1
// }

// struct  Rectangle {
//    width: i32,
//    height: i32
// }

// fn main() {
//    let rect1 = Rectangle {
//       width: 30,
//       height: 50
//    };

//    println!("The area is: {}", area(rect1));
// }

// fn area(rectangele: Rectangle) -> i32{
//    rectangele.width * rectangele.height
// }

// fn main() {
//     let s = String::from("Prados World");

//    let first_word = first_word(&s);

//    println!("{}", first_word);

// }

// fn first_word(s: &str) -> &str {
//    let bytes = s.as_bytes();

//    for (i, &item) in bytes.iter().enumerate() {
//       if item == b' ' {
//          return &s[..i];
//       }
//    }

//    &s[..]
// }

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }

//     // methods with the same name as a field
//     fn width(&self) -> bool {
//       self.width > 0
//     }

//     // getter method of the height
//     fn height(&self) -> u32 {
//       self.height
//     }

//     // check if a rectangle can hold another
//     fn can_hold(&self, other: &Rectangle) -> bool {
//       self.width > other.width && self.height > other.height ||
//       self.width > other.height && self.height > other.width
//     }

//     // associated function to define a square
//     fn square(size: u32) -> Self {
//       Self {
//          width: size,
//          height: size
//       }
//     }
// }

// fn main() {
//    let square: Rectangle = Rectangle::square(10);

//    println!("The area of the square is: {}", square.area());
//    println!("{:?}", square);
// }

// #[derive(Debug)]
// enum IpAddrKind {
//    v4,
//    v6,
// }

// fn main() {
//     let four = IpAddrKind::v4;
//     let six = IpAddrKind::v6;

//     route(four);
//     route(six);

// }

// fn route(ip_type: IpAddrKind) {
//     println!("Route called with {:?}", ip_type);
// }

// #[derive(Debug)]
// enum TrafficLight {
//    Red,
//    Yellow,
//    Green,
// }

// fn print_light(light: TrafficLight) {
//    match light {
//       TrafficLight::Red => println!("Stop"),
//       TrafficLight::Yellow => println!("Slow down"),
//       TrafficLight::Green => println!("Go"),
//    }
// }

// fn main() {
//    let current_light = TrafficLight::Green;
//    print_light(current_light);
// }

// enum Message {
//     Text(String),
//     Number(i32),
// }

// fn show_message(msg: Message) {
//    match msg {
//       Message::Text(s) => println!("{}", s),
//       Message::Number(s) => println!("{}", s),
//    }
// }

// fn main() {
//    let msgI = Message::Text(String::from("Hello"));
//    let msgII = Message::Number(4);

//    show_message(msgI);
//    show_message(msgII);
// }

// #[derive(Debug)]
// enum IpAddr {
//    v4(u8, u8, u8, u8),
//    v6(String),
// }

// fn main() {

//    let home = IpAddr::v4(127, 0, 0, 1);
//    let r#loop = IpAddr::v6(String::from("::1"));

//    println!("{:?}", home);
//    println!("{:?}", r#loop);
// }

// #[derive(Debug)]
// enum IpAddr {
//    v4(u8, u8, u8, u8),
//    v6(String),
// }

// fn main() {

//    let home: IpAddr = IpAddr::v4(127, 0, 0, 1);
//    println!("{:?}", home);

//    let loopback: IpAddr = IpAddr::v6(String::from("::1"));
//    println!("{:?}", loopback);

// }

// #[derive(Debug)]
// enum Message {
//    Quit,
//    Move {x: i32, y: i32},
//    Write (String),
//    ChangeColor (i32, i32, i32),
// }

// impl Message {
//    fn call(&self) {
//       println!("{:?}", self);
//    }
// }

// fn main() {

//    let m = Message::Write(String::from("Hello, world!"));
//    let x = Message::Move { x: 3, y: 4 };
//    let y = Message::ChangeColor(0, 0, 0);
//    let z = Message::Quit;

//    m.call();
//    x.call();
//    y.call();
//    z.call();

// }

// enum Option<T> {
//    Some(T),
//    None,
// }

// fn main() {
//    let x: i8 = 5;
//    let y: Option<i8> = Some(4);

//    //try to sum them
//    let sum = x + y.unwrap();
//    println!("Sum: {}", sum);
// }

// #[derive(Debug)]
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(Rarity),
// }

// #[derive(Debug)]
// enum Rarity {
//    Common, 
//    Uncommon,
//    Rare,
//    Epic,
//    Legendary,
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => {
//             println!("Lucky you");
//             1
//         }
//         Coin::Nickel => {
//             println!("Lucky you");
//             5
//         }
//         Coin::Dime => {
//             println!("Lucky you");
//             10
//         }
//         Coin::Quarter(rarity) => {
//             println!("You got a {:?} quarter!", rarity);
//             25
//         }
//     }
// }

// fn main() {
//     let coin = Coin::Quarter(Rarity::Epic);
//     println!("Value in cents: {}", value_in_cents(coin))
// }

// fn main() {
//     fn plus_one(x: Option<i32>) -> Option<i32> {
//         match x {
//             None => None,
//             Some(i) => Some(i + 1),
//         }
//     }

//     let five = Some(5);
//     let six = plus_one(five);
//     println!("{:?}", six);
// }

// fn main() {
//    let dice_roll = 9;

//    match dice_roll {
//        1 => println!("You rolled a one!"),
//        2 => println!("You rolled a two!"),
//        3 => println!("You rolled a three!"),
//        4 => println!("You rolled a four!"),
//        5 => println!("You rolled a five!"),
//        6 => println!("You rolled a six!"),
//        _ => println!("You rolled something else!"),
//    }
// }







// fn main() {
//    let config_max = Some(100);

//    // match config_max {
//    //     Some(max) => println!("The max is configured to be {}", max),
//    //     _ => (),
//    // }


//    // use an if let statement to check if the value is Some
//    if let Some(max) = config_max {
//       println!("The max is configured to be {}", max)
//    }

// }

// #[derive(Debug)]
// struct  User {
//    active: bool,
//    username: String,
//    email: String,
//    sign_in_count: u8,
// }

// fn main() {
//     let user1 = buid_user(String::from("edohemmanuel4real@gmail.com"), String::from("EDOHWARES"));

//     println!("{:?}", user1);

//     let user2 = User {
//       email: String::from("user2@gmail.com"),
//       ..user1
//     };

//     println!("{:?}", user2);
//     println!("{:?}", user1);
// }

// fn buid_user(email: String, username: String) -> User {
//    return User {
//       active: true,
//       username,
//       email,
//       sign_in_count: 1,
//    }
// }

// fn main() {
//     let width: usize = 30;
//     let height: usize = 20;

//     println!("{}", calculate_area(width, height));
// }

// fn calculate_area(width: usize, height: usize) -> usize{
//    width * height
// }

// fn main() {
//     let rect1: (usize, usize) = (50, 30);

//     println!("{}", calculate_area(rect1));
// }

// fn calculate_area(dimension: (usize, usize)) -> usize {
//    dimension.0 * dimension.1
// }

// #[derive(Debug)]
// struct Rectangle {
//    width: usize,
//    height: usize,
// }

// impl Rectangle {
//     fn area(&self) -> usize {
//         self.width * self.height
//     }

//     fn width(&self) ->bool {
//         self.width > 0
//     }
// }

// fn main() {
//    let scale: usize = 2;
//    let rect1: Rectangle = Rectangle {
//       width: dbg!(40 * &scale),
//       height: 30,
//    };

//    println!("The area of rect1 is: {}", rect1.area());


// }



// main.rs
// use std::collections::HashMap;
// use rand::Rng;

// fn main() {
//     let mut map = HashMap::new();
//     map.insert("key1", "value1");
//     map.insert("key2", "value2");

//     println!(
//       "{:?}", map
//     );

//     let secret_number = rand::rng().random_range(1..=100);
//     println!("{}", secret_number);
// }

// Collections are data structures that store multiple values.
/* 
They are stored on the heap,
the data does not need to be known at compile time,
and can grow or shring as the program runs.

A Vector allow you to store a variable number of values next to each other
*/

use std::vec;

fn main() {
  // // create a vector
  // let vec1: Vec<i32> = Vec::new();
  // println!("{:?}", vec1);

  // // create a vector with initial values
  // let vec2 = vec![1, 2, 3, 4, 5];
  // println!("{:?}", vec2);

  // // update a vector
  // let mut vec3: Vec<i32> = Vec::new();

  // vec3.push(1);
  // vec3.push(2);
  // vec3.push(4);
  // println!("{:?}", vec3);

  // vec3.pop();
  // println!("{:?}", vec3);

  // accessing element in a vector
  // 1. Reading elements
  let vec4: Vec<char> = vec!['a', 'b', 'c', 'd', 'e'];


  // using the square bracket method
  // let fourth: char = vec4[4];

  // using the get method
  let fourth: Option<&char> = vec4.get(40);

  match fourth {
      Some(&fourth) => println!("The fourth value is: {}", fourth),
      None => println!("The value doesn't exists...")
  }

  // iterating over the values in a vector
  let vec5: Vec<i32> = vec![1, 2, 3, 4, 5];
  for i in &vec5 {
    println!("{}", i);
  }

  // iterating over mutable references
  let mut vec6 = vec![10, 20, 30, 40, 50];
  for i in &mut vec6 {
    *i += 1;
  };

  println!("{:?}", vec6);

  // store multiple types in a vector using enums
  #[derive(Debug)]
  #[allow(dead_code)]
  enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
  }

  let row: Vec<SpreadSheetCell>= vec![
    SpreadSheetCell::Int(3),
    SpreadSheetCell::Float(10.12),
    SpreadSheetCell::Text(String::from("Blue")),
    SpreadSheetCell::Int(4),
    SpreadSheetCell::Int(5)
  ];

  println!("{:?}", row);

}