pub mod fibonnaci;
pub mod summary;

fn main() {

    let mut num: i8 = -123; // i32 - 32 bit , i = signed, u = unsigned (only positive), mut = mutability
    let unsigned_num: u16 = 55; // in-case of overflow, the complier will throw error
    let float_num = 0.55;

    for _ in 0..1000 { // No error in compilation for run time logic of overflow
        // num -= 100
        num = 101
    };

    println!("num: {}, unsignedNum: {}, floatingNum: {}", num, unsigned_num, float_num);

    let _string_1 = "First Rust Project";
    let mut string_2 = String::from("Hello");

    for _ in 0..4 {
        string_2 = string_2 + " Okay";
    }

    println!("{}", string_2);
    
    let nth_char = string_2.chars().nth(11);

    match nth_char {
        Some(c) => println!("{}", c),
        None => println!("N'th Char of {} is undefined", string_2)
    }

    println!("{}", nth_char.unwrap()); // this will throw an runtime err if it's undefined - so avoid

    let is_even = unsigned_num % 2 == 0;

    if is_even {
        println!("{} Is Even", unsigned_num)
    } else {
        println!("Not Even : {}", unsigned_num)
    }

    for i in 0..5 {
        print!("{} ", i);
    }
    println!();

    let sentence = String::from("Get First Word of this Sentence");
    // search if we can mandate our own convention for camel case declaration of variable names?
    let first_word = get_first_word(sentence);
    println!("First Word is : `{}`", first_word);

    // heap 
    string_fn();

    // ownership
    create_str();
    auto_transfer_owner();
    return_ownership();

    // borrowing & references
    /* String / Vector's can be passed as borrowed without change in ownership
    so the original owner still remains the owner & it can be given as borrow to multiple 
    but if any ops needs to be done then only 1 borrower */
    reference_str();
    update_str();

    // structs
    user_structs();
    // impl structs
    cal_rect_area();

    // enums
    enum_directions();
    // pattern matching in enums
    calculate_shape_area();

    // generics
    generics_error_handling();

    // option enum
    option_enum_null_handling();

    // libs 
    random_num();
    current_time();

    // collections
    vectors_fn(); // vectors
    maps_fn(); // hashmaps

    // iterators
    iterators_fn();

    fibonnaci_fn(); // hashmaps , referencing , borrowing etc

    // generics
    generics_fn(); // to reduce code duplications

    // traits
    traits_fn(); // defining similar behaviour

    // generic lifetime
    generic_lifetime_fn();
}

fn get_first_word(sentence: String) -> String {
    let mut first_word = String::from("");

    for char in sentence.chars() {
        if char == ' ' {
            break;
        }
        first_word.push(char);
    }

    return first_word;
}

fn string_fn() {
    let mut s = String::from("HELLO");

    println!("Initially: {}", s);
    // :p -> Pointer, :d -> debug data
    println!("Cap: {}, Len: {}, Pointer: {:p}", s.capacity(), s.len(), s.as_ptr());

    for _ in 0..5 { // increase to 1000 & see the change in pointer
        s.push_str( " WORLD");

        // Cap & Len also changes, Pointer changes only when 
        // the entire data needs to be shifted incase big change is made
        // Capaity can increase more than Len when to allocate memory in-advance to a frequently used var
        println!("Cap: {}, Len: {}, Pointer: {:p}", s.capacity(), s.len(), s.as_ptr());
    }

    // println!("Later: {}", s);
    // println!("Cap: {}, Len: {}, Pointer: {:p}", s.capacity(), s.len(), s.as_ptr());
} 

fn create_str() {
    let s1 = String::from("Sample String in Heap");
    println!("{} owned by s1", s1);

    let s2 = s1; // s1's ref will pass to s2 (new owner) and s1 will cease to exists
    // println!("{}, {}", s1, s2); borrow of moved value: `s1`
    println!("{} owned by s2", s2);
}

fn auto_transfer_owner() {
    let s1 = String::from("S1 in Heap");
    let n1 = 444;

    // s1 ownership transferred - pass by reference (For String & Vectors)
    // n1 passed by value - since it's an integer
    takes_ownership(s1, n1);  // can do s1.clone() if needed to copy without passing s1's ref

    // println!("{}", s1); // borrow of moved value: `s1`
    println!("Original Number {}", n1);
}

fn takes_ownership(new_string: String, new_number: i32) {
    println!("New Owner of string: {}, new number: {}", new_string, new_number)
}

fn return_ownership() {
    let mut s1 = String::from("Sample String of S1");
    println!("First Owner of : {}", s1);

    s1 = take_and_return_ownership(s1); // pass by reference was done but ref. was again passed back
    println!("Back to First Owner : {}", s1);
}

fn take_and_return_ownership(new_string: String) -> String{
    println!("New Owner of : {}", new_string);
    return new_string
}

fn reference_str() {
    let s1 = String::from("New in Heap");
    let s2 = &s1; // s1's ref will pass to s2 (but not ownerership)

    println!("{} owned by s1", s1); // no error of borrow of moved value: `s1`
    println!("{} owned by s1 but also borrowed by s2", s2);
}

fn update_str() {
    let mut s1 = String::from("Mutable String with Owner");

    let s2 = &mut s1;
    s2.push_str(" 2nd Borrower ");
    println!("{}", s1);

    let s3 = &mut s1;
    s3.push_str(" 3rd Borrower ");
    println!("{}", s1);

    pass_mut_ref(&mut s1);

    // mutable ref can only be passed to one person at a time - so when &mut s1 is passed to s3 & then to function - s2 & s3 ceases to exist
    // println!("{}, {}", s2, s3);
    println!("{}", s1);
}

// borrowing with mutability
fn pass_mut_ref(s: &mut String) {
    s.push_str(" Addition in borrowed String");
}

// #[derive(Debug)]
struct User {
    name: String,
    age: u32, // unsigned - only positive
    active: bool,
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(Name: {}, Age: {}, Active: {})", self.name, self.age, self.active)
    }
}

fn user_structs() {
    let user = User {
        name: String::from("vishesh"),
        age: 23,
        active: true
    };

    // println!("{:?}", user); --> works with // #[derive(Debug)]
    println!("{}", user);
}

struct Rect {
    width: u32,
    length: u32
}

impl Rect {
    fn area(&self) -> u32 {
        return self.width * self.length;
    }
    fn perimeter(&self) -> u32 {
        return 2 * (self.width + self.length);
    }
    fn debug() -> u32 { // acts as a static fn, can't be called on Rect object but directly on Rect struct
        return 1
    }
}

fn cal_rect_area() {
    let rect: Rect = Rect {
        width: 20,
        length: 10
    };

    println!("Area of the Rect is {}", rect.area());
    println!("Perimeter of the Rect is {}", rect.perimeter());
    println!("Debug - Static Func. -- printing: {}", Rect::debug());
}

#[allow(dead_code)] // helps you ignore unused codes in compilation
enum Direction {
    East,
    West,
    North,
    South
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::East => write!(f, "EAST"),
            Self::West => write!(f, "WEST"),
            Self::North => write!(f, "NORTH"),
            Self::South => write!(f, "SOUTH")
        }
    }
}

fn enum_directions() {
    let new_direction = Direction::North;
    move_around(new_direction);
}

fn move_around (new_direction: Direction) {
    println!("Moving to new direction {}", new_direction)
}

#[allow(dead_code)]
enum Shape {
    Circle(f64),
    Square(f64),
    Rectangle(f64, f64)
}

#[derive(Debug)]
struct NameArea {
    name: String,
    area: f64
}

impl Shape {
    fn _area(&self) -> f64 {
        match self {
            Self::Circle(radius) => return std::f64::consts::PI * radius * radius,
            Self::Square(side) => return side * side,
            Self::Rectangle(length, width) => return length * width
        }
    }

    fn _display(&self) -> String {
        match self {
            Self::Circle(_radius) => return String::from("CIRCLE"),
            Self::Square(_side) => return String::from("SQUARE"),
            Self::Rectangle(_length, _width) => return String::from("RECTANGLE")
        }
    }

    // combo function
    fn name_area(&self) -> NameArea {
        match self {
            Self::Circle(radius) => return NameArea {
                name: String::from("CIRCLE"),
                area: std::f64::consts::PI * radius * radius
            },

            Self::Square(side) => return NameArea {
                name: String::from("SQUARE"),
                area: side * side,
            },
            Self::Rectangle(length, width) => return NameArea {
                name: String::from("RECTANGLE"),
                area: dbg!(length * width)
            }
        }
    }
}

fn calculate_shape_area() {
    let shape = Shape::Rectangle(5.4, 9.8);
    // added a debugger to print the execution at this point of control
    let get_name_area = dbg!(shape.name_area());

    // println!("Area of the shape {}, is {:.2}", shape.display(), shape.area()) // rounded upto .xx decimal
    println!("Area of the shape {} is {:.2}", get_name_area.name, get_name_area.area)
}

/*
// generics
struct Point<T> {
    // both x & y needs to be of same type - any time but same type
    x: T,
    y: T
}

enum Result<A, B> {
    Ok(A),
    Err(B),
}
*/

fn generics_error_handling() {
    let mut path = String::from("content/example_context.txt");
    let mut res = std::fs::read_to_string(&path);

    for iter in 0..2 {
        match dbg!(&res) { // added debugger for experiment
            Ok(content) => {
                println!("File Found - content: {}", content)
            },
            Err(error) => {
                println!("File not found at path: {}, err: {}", &path, error)
            }
        }

        path.push_str("_1"); // changing path
        res = std::fs::read_to_string(&path); // again opening file reading

        println!("Execution No. {} Completed Successfully", iter + 1)
    }
}

fn option_enum_null_handling() {
    let s = String::from("HELL");

    match find_first_a(s) {
        Some(index) => println!("Found at {}-index", index),
        None => println!("'a' Not found")

    }
}

fn find_first_a(s: String) -> Option<i32>{
    for (index, character) in s.to_lowercase().chars().enumerate() {
        if character == 'a' {
            return Some(index as i32)
        }
    }
    // can use Result generics for error handling aswell
    return None;
}

// rust libs 
use rand::{Rng, thread_rng};

fn random_num() {
    let mut rng = thread_rng();
    let n: u32 = rng.gen_range(10..=1000);
    println!("Random Number: {}", n);
}

// DATE Time Rust Lib
use chrono::{Utc, Local};

fn current_time() {
    let now = Local::now();
    let utc_now = Utc::now();
    println!("Local Time is : {}", now);
    println!("UTC Time is : {}", utc_now);
}


fn vectors_fn() {
    let mut v:Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];

    // same as string - dynamic memory allocation
    for i in 0..5 {
        v.push(i);
        println!("Cap: {}, Len: {}, Pointer: {:p}", v.capacity(), v.len(), v.as_ptr());
    }

    println!("Original Vector : {:?}", &v);
    let even_v: Vec<&i32> = get_even_elements(&v);

    println!("All Even elements of the vector : {:?}", even_v);

    // handles index out of bound
    match v.get(20) {
        Some(third) => println!("The third index value is : {}", third),
        None => println!("Index doesn't exists") 
    }
    
    // will break at runtime if index is overflown
    println!("v 1st Index: {}", &v[0]);

    // can't use immutable reference taken before taking a mutable reference, opposite would work
    /*
    let second  = &v2[1]; // immutable reference of v2 
    v2.push(4); // taking a mutable reference and pushing it
    println!("Second Index of v2: {}", second); // trying to print the immutable reference after */

    println!("v2 vector: {:?}", v2)
}

fn get_even_elements(v: &Vec<i32>) -> Vec<&i32> {
    let mut even_v = Vec::new();


    for element in v.iter() {
        if element % 2 == 0{
            even_v.push(element);
        }
    }
    return even_v
}

use std::{collections::HashMap};
fn maps_fn(){
    let mut cache = HashMap::new();

    cache.insert(String::from("yellow"), 5);
    cache.insert(String::from("blue"), 9);

    println!("Map in rust: {:?}", cache);

    let mut score_of_yellow = cache.get(&String::from("yellow"));

    match score_of_yellow {
        Some(value) => println!("Score of yellow : {}", value),
        None => println!("Score of Yellow not found in Cache Hashmap")
    };

    // if entry exists do nothing - if doesn't insert
    cache.entry(String::from("blue")).or_insert(12);

    // update entry
    cache.insert(String::from("yellow"), 19);

    let score_of_blue = cache.get(&String::from("blue"));

    match score_of_blue {
        Some(value) => println!("Score of blue : {}", value),
        None => println!("Score of Blue not found in Cache Hashmap")
    };

    score_of_yellow = cache.get(&String::from("yellow"));

    match score_of_yellow {
        Some(value) => println!("Updated Score of yellow : {}", value),
        None => println!("Score of Yellow not found in Cache Hashmap")
    };
}

fn iterators_fn() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for element in v1_iter {
        println!("Iterating , got {}", element)
    }

    println!("v1 is still valid, as it was only borrowed, {:?}", v1);

    // mutable iterators
    let mut v2 = vec![4, 5, 6];
    println!("Old V2 : {:?}", v2);

    let v2_iter = v2.iter_mut();
    for mutable_element in v2_iter {
        print!("Got: {}, ", mutable_element);
        *mutable_element += 3;
        println!("updated to : {}", mutable_element)
    }
    println!("New V2: {:?}", v2);

    // into iter - takes ownership
    let v3 = vec![10, 11, 12];
    let v3_iter = v3.into_iter();

    for owned_element in v3_iter {
        println!("Got : {}", owned_element)
    }

    // println!("Cant Print v3 as its ownership is moved : {:?}", v3);

    // default iter functions
    let v4 = vec![15, 17, 19];
    let sum_v4: i32 = v4.iter().sum(); // consuming adaptor

    println!("Sum of {:?} is {}", v4, sum_v4);

    // iterator adaptor - non consuming

    let v5 = vec![31, 23, 29];
    let v5_iter = v5.iter();
    // take x return x + 1, works like lambda functions in python
    let map_iter = v5_iter.map(|x| x + 1); 

    for iter_element in map_iter {
        println!("Got x+1, - {}", iter_element)
    }

    let filter_iter = v5.iter().filter(|x| x <= &&30);

    for filter_iter_element in filter_iter {
        println!("Got x <= 30, - {}", filter_iter_element)
    }

    let v6 = vec![12, 13, 14, 15, 16, 17];

    let filtered_v6_odd = v6.iter().filter(|x| *x % 2 != 0);
    let mut double_odd_v6: Vec<i32> = Vec::new();

    for element in filtered_v6_odd.into_iter() {
        double_odd_v6.push(element * 2);
    }

    println!("{:?} filtered odd values & doubled it - {:?}", v6, double_odd_v6);


    let v7 = vec![1, 2, 34, 13, 17];
    let odd_double_v7 = v7.iter().filter(|x| *x % 2 != 0).map(|x| x * 2);
    let vec_v7: Vec<i32> = odd_double_v7.collect();

    println!("Same thing using map + collect (for iter to vec): {:?}", vec_v7);

}

fn fibonnaci_fn () {
    let num = 30;
    let fib = fibonnaci::Fibonnaci;
    println!("Fibonnaci of the number: {}, is {}", num, fib.fib(num));
}

// to reduce code duplications - using generics
fn largest<T: PartialOrd + Copy>(vec: &Vec<T>) -> T {
    /* Traits: 
    PartialOrd -> Something that can be orderd
    Copy: Something that can be copied */
    let mut max = vec[0];

    for i in vec {
        if *i > max {
            max = *i;
        } 
    }
    return max;
}

fn generics_fn() {
    let mut vec_num = Vec::new();
    for i in 1..8 {
        vec_num.push(i * 12)
    }

    let largest_num = largest(&vec_num);
    println!("Largest element in vector: {:?} is {}", vec_num, largest_num);

    let vec_char = vec!['a', 'd', 'v', 'm', 'g'];
    let largest_char = largest(&vec_char);
    println!("Largest element in vector: {:?} is {}", vec_char, largest_char);
}

fn traits_fn () {
    let news_article = summary::Article {
        author: String::from("Shakespeare"),
        headline: String::from("Thou shall pass"),
        content: String::from("value")
    };

    let tweet = summary::Tweet {
        author: String::from("@johnwick"),
        content: String::from("car is not for sale"),
        reply: false,
        retweet: true
    };

    // println!("Summary of the Article: {}", news_article.summarize());
    // println!("Summary of the Tweet: {}", tweet.summarize());

    notify_fn(&tweet);
    notify_fn(&news_article);
}

// fn notify_fn(item : &impl summary::Summary) { // this too will work, using <Generics> below
fn notify_fn<T: summary::Summary>(item :&T) {
    println!("Breaking News: {}", item.summarize())
}

fn generic_lifetime_fn() {
    let s1 = String::from("hellow");
    {// ------------------------------------------------------------------------
        let s2 = String::from("world");                              //| lifetime block
        let result = longest_str(&s1.as_str(), &s2.as_str());     //| s2 & result are only accessible inside this {}
        println!("Largest string : {:?}", String::from(result))              //|
    }// ------------------------------------------------------------------------
}

/*
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime */
fn longest_str<'a>(x: &'a str, y: &'a str) -> &'a str {
    // what we are telling here is the result will be having a lifetime which is the smallest of x & y's lifetimes
    if x.len() > y.len() {
        return x;
    }

    return y;
}


