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