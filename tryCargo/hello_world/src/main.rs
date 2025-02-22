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

fn main() {
    // CUSTOM DATA TYPES
        // STRUCTS
    struct Person {
        name: String,
        age: u8,
    }

    struct Animal {
        name: String,
        number_of_legs: u8,
        breed: (String, String, String),
        age: f32,
        male: bool,
    }

    let person = Person {
        name: String::from("Edoh"),
        age: 20,
    };

    let my_pet = Animal {
        name: String::from("Bingo"),
        number_of_legs: 4,
        breed: (String::from("Lassa"), String::from("German Shepherd"), String::from("Chaw-Chaw")),
        age: 3.0,
        male: false,
    };

    println!("His name is {} and age is {}", person.name, person.age);
    println!("{} has {} legs and is {}years old, and is a {}. It's a cross breed of: {}, {}, {}", 
    my_pet.name, 
    my_pet.number_of_legs, 
    my_pet.age, 
    if my_pet.male 
    {
        "Male"
    } else {
        "Female"
    }, 
    my_pet.breed.0,
    my_pet.breed.1,
    my_pet.breed.2,
)

}

