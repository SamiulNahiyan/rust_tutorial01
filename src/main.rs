#![allow(unused)]

mod modules;

use rand::Rng;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::f64::consts::PI;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write,Error};
use std::thread;
use std::time::Duration;

use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};

fn main() {
    println!("What is your Name?");
    let mut name: String = String::new();
    let greeting: &str = "nice to meet you";
    io::stdin()
        .read_line(&mut name)
        .expect("didn't receive input");
    println!("Hello {}! {}", name.trim_end(), greeting);
    // another type of variable
    const ONE_MIL: u32 = 1_000_000;
    println!("{}", ONE_MIL);
    // how can we define float
    const PI: f32 = 3.141;
    // we need to define char with single quotes and string with double quotes
    let age: &str = "42";
    // we can define variable with the same name with different data types
    // here u32 means unsigned 32 bit
    let mut age: u32 = age.trim().parse().expect("age wasn't assigned as number");
    age = age + 1;
    println!("i am {} and i want $ {}", age, ONE_MIL);

    // data types
    // unsigned integer : u8, u16, u32, u64, u128, usize
    // singed integer : i8, i16, i32, i64, i128, isize
    println!("Max u32 : {}", u32::MAX);
    println!("Max u64 : {}", u64::MAX);
    println!("Max usize : {}", usize::MAX);
    println!("Max u128 : {}", u128::MAX);
    println!("Max f32 : {}", f32::MAX);
    println!("Max f64 : {}", f64::MAX);

    // another data type is booleans
    let is_true: bool = true;
    let my_grade = "A";

    // math
    let num_1: f32 = 1.111111111111111;
    println!("f32:{}", num_1 + 0.111111111111111);
    let num_2: f32 = 1.111111111111111;
    println!("f64:{}", num_2 + 0.111111111111111);
    let mut num_3: u32 = 5;
    let num_4: u32 = 4;
    println!("5 + 4 : {}", num_3 + num_4);
    println!("5 - 4 : {}", num_3 - num_4);
    println!("5 / 4 : {}", num_3 / num_4);
    println!("5 * 4 : {}", num_3 * num_4);
    println!("5 % 4 : {}", num_3 % num_4);
    num_3 += 1;

    // generate random values
    let random_num = rand::thread_rng().gen_range(1..101);
    println!("random : {}", random_num);

    // if statement
    let age2: i32 = 8;
    if (age2 >= 1) && (age2 <= 18) {
        println!("important birthday");
    } else if (age2 == 21) || (age2 == 50) {
        println!("IMPORTANT BIRTHDAY ");
    } else if age2 >= 65 {
        println!("Important birthday");
    } else {
        println!("moira jan bro");
    }
    // ternary operator
    let mut my_age = 47;
    let can_vote = if my_age >= 18 { true } else { false };
    println! {"can vote : {} ", can_vote}
    // match
    let age3 = 8;
    match age3 {
        1..=18 => println!("Important birthday"),
        21 | 50 => println!("IMPORTANT BIRTHDAY"),
        65..=i32::MAX => println!("Important Birthday"),
        // _ means = match everything else
        _ => println!("Not an important birthday"),
    };
    // another example of match
    let my_age2 = 18;
    let voting_age2 = 18;
    match my_age2.cmp(&voting_age2) {
        Ordering::Less => println!("you gained the right to vote"),
        Ordering::Greater => println!("you don't have the permission to vote"),
        Ordering::Equal => println!("you gained the right to vote"),
    }

    // arrays
    // elements of an array must be of same type
    // and a array has the same size
    let arr_1 = [1, 2, 3, 4, 5, 6, 7];
    println!("1st : {}", arr_1[01]);
    println!("1st : {}", arr_1.len());
    let arr_2 = [1, 2, 3, 4, 5];
    let mut loop_idx = 0;
    // loop{
    //     if arr_2[loop_idx] % 2 == 0{
    //         loop_idx += 1;
    //         continue;
    //     }
    //     if arr_2[loop_idx] == 9{
    //         break;
    //     }
    //     println!("val : {}", arr_2[loop_idx]);
    //     loop_idx += 1;
    // }
    // while loop
    // while loop_idx < arr_2.len(){
    //     print!("arr : {}", arr_2[loop_idx])
    // }
    // // for loop
    // for val in arr_2.iter(){
    //     println!("val : {}",val);
    //     break;
    // }
    // tuples
    let my_tuple: (u8, String, f64) = (47, "Nahiyan".to_string(), 50_000.00);
    println!("Name {}", my_tuple.1);
    // assign multiple tuples
    let (v1, v2, v3) = my_tuple;
    println!("Age : {}", v1);

    // strings
    // there are two types of string
    // String  --> vector of bytes that can be change
    // &str  --> and string type which is going to point to the string
    // and allow for viewing of said string
    let mut st1 = String::new();
    st1.push('A');
    st1.push_str("words");
    for word in st1.split_whitespace() {
        println!("{}", word);
    }
    // if you like to replace a string
    let st2 = st1.replace("A", "Another");
    println!("{}", st2);

    // string of just random different char
    let st3 = String::from("x r t b h k k a m c");
    let mut v1: Vec<char> = st3.chars().collect();
    v1.sort();
    v1.dedup();
    for char in v1 {
        println!("{}", char);
    }
    let st4: &str = "Random string";
    let mut st5: String = st4.to_string();
    println!("{}", st5);
    let byte_arr1 = st5.as_bytes();
    let st6 = &st5[0..6];
    println!("string length {}", st6.len());
    st5.clear();
    // combine string
    let st6 = String::from("just some");
    let st7 = String::from("words");
    let st8 = st6 + &st7;
    for char in st8.bytes() {
        println!("{}", char);
    }
    // casting
    let int_u8: u8 = 5;
    let int2_u8: u8 = 4;
    let int3_u32: u32 = (int_u8 as u32) + (int2_u8 as u32);

    // Enums
    enum DayName {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    }
    // . is used when you have a value on the left-hand-side. :: is used when you have a type or module. Or: . is for value member access, :: is for namespace member access.
    impl DayName {
        fn is_weekend(&self) -> bool {
            match self {
                DayName::Saturday | DayName::Sunday => true,
                _ => false,
            }
        }
    }
    let today = DayName::Monday;
    match today {
        DayName::Monday => println!("Everyone Hates Monday"),
        DayName::Tuesday => println!("Everyone Hates Tuesday"),
        DayName::Wednesday => println!("Everyone Hates Wednesday"),
        DayName::Thursday => println!("Everyone Hates Thursday"),
        DayName::Friday => println!("Everyone Hates Friday"),
        DayName::Saturday => println!("Saturday is the fking error"),
        DayName::Sunday => println!("Everyone Hates Sunday"),
    }
    println!("is today the weekend {}", today.is_weekend());

    // vectors
    // like array but they can grow if mutable
    // and stores data as same type
    let vec1: Vec<i32> = Vec::new();
    let mut vec2 = vec![1, 2, 3, 4];
    vec2.push(5);
    // print from specific index
    println!("1st : {}", vec2[0]);
    // verify value exists
    let second: &i32 = &vec2[1];
    match vec2.get(1) {
        Some(second) => println!("2nd : {}", second),
        None => println!("No 2nd value"),
    }
    // we can cycle and can change value
    for i in &mut vec2 {
        *i *= 2;
    }
    for i in &vec2 {
        println!("{}", i);
    }
    println!("Vec length {}", vec2.len());
    println!("Pop : {:?}", vec2.pop());

    // function called here
    // get_some(5,4);
    println!("{}", get_some_2(5, 4));

    // get the two value from the function
    let (val_1, val_2) = get_some_3(3);
    println!("Nums {} {} ", val_1, val_2);

    // function and list
    let num_list = vec![1, 2, 3, 4, 5];
    println!("sum of list = {}", sum_list(&num_list));

    // generic function print
    println!("5 + 4 = {}", get_sum_gen(5, 4));
    println!("5.2 + 4.6 = {}", get_sum_gen(5.2, 4.6));

    // ownership check 01
    owner_ship();

    // print_str(st1);
    let str4 = print_return_str(st1);
    println!("str3 = {}", st3);

    let mut str_1 = String::from("Nahiyan ");
    change_string(&mut str_1);

    // hashMaps function
    // hash maps are going to be used to store key value pairs
    let mut heroes = HashMap::new();
    // k is the key and v is the value
    heroes.insert("Super Man ", "nigga");
    heroes.insert("Bat-Man ", "rich kid");
    heroes.insert("Wonder Woman  ", "old lady");

    // we can iterate over hashmaps
    for (k, v) in heroes.iter() {
        println!("{} = {}", k, v)
    }
    // println!("heroes {}", heroes.len());
    if heroes.contains_key(&"Bat-Man") {
        let the_batman = heroes.get(&"Bat-Man");
        match the_batman {
            Some(x) => println!("He is a hero"),
            None => println!("he is gay"),
        }
    }

    // struct
    struct Customer {
        name: String,
        address: String,
        balance: f32,
    }

    let mut bob: Customer = Customer {
        name: String::from("hacker boii"),
        address: String::from("aayy alakai aayy tui"),
        balance: 420.69,
    };

    // change a value
    bob.address = String::from("420 road 69no. house");

    struct Rectangle<T, U> {
        length: T,
        height: U,
    }
    let rec = Rectangle {
        length: 4,
        height: 10.5,
    };
    trait Shape {
        fn new(length: f32, width: f32) -> Self;
        fn area(&self) -> f32;
    }
    struct Rectange01 {
        length01: f32,
        width01: f32,
    }
    struct Circle {
        length02: f32,
        width02: f32,
    }

    // for rectangle
    impl Shape for Rectange01 {
        fn new(length01: f32, width01: f32) -> Rectange01 {
            return Rectange01 { length01, width01 };
        }
        fn area(&self) -> f32 {
            return self.length01 * self.width01;
        }
    }

    // for circle
    impl Shape for Circle {
        fn new(length02: f32, width02: f32) -> Circle {
            return Circle { length02, width02 };
        }
        fn area(&self) -> f32 {
            return (self.length02 / 2.0).powf(2.0) * PI;
        }
    }

    // we can refer to this as if they are shapes
    let rec: Rectange01 = Shape::new(10.0, 10.0);
    let circ: Circle = Shape::new(10.0, 10.0);
    println!("rec area : {} ", rec.area());
    println!("circle area : {} ", circ.area());

    // calling modules
    order_food();

    // error handling
    // Result has 2 variants Ok and Err
        // enum Result<T, E>{
        // OK(T),
        // Err(E),
        // }
        // Where T represents the data typeof the value returns and E 
        //the type of error
    // let lil_arr:[i32;2] = [1,2];
    // println!("{}", lil_arr[1]);
    // panic!("Terrible Error");
    // real world ex. with error handling
    // let path = "lines of texts";
    // let output = File::create(path);
    // let mut output = match output {
    //     Ok(file) => file,
    //     Err(error) => {panic!("problem creating file {:?}", error);}
    // };
    // write!(output,"just some\nrandom world").expect("failed to write to file");
    // let input = File::open(path).unwrap();
    // let buffered = BufReader::new(input);
    // for line in buffered.lines(){
    //     println!("{}",line.unwrap());
    // }
    // let output2 = File::create("rand.txt");
    // let output2 = match output2{
    //     Ok(file) => file,
    //     Err(error) => match error.kind(){
    //         ErrorKind::NotFound => match File::create("rand.txt"){
    //             Ok(fc) => fc,
    //             Err(e) => panic!("cant create file : {:?}", error)
    //         },
    //         _other_problem => panic!("problem opening file {:?}", error),
    //     },
    // };

    // iterators
    let mut arr_it = [1,2,3,4];
    for val in arr_it.iter(){
        println!("{}", val);
    }
    // we can consume the collection but we cannot use the collection
    arr_it.into_iter();

    // we can create and iterator 
    let mut iter1 = arr_1.iter();
    println!("1st {:?}",iter1.next());

    // closers
    // let var_name = |parameter| -> return_type {code}

    // basic type of closer
    let can_vote1 = |age2: i32| {
        age >= 18
    };
    println!("can vote {}", can_vote1(8));

    // complicated closer
    // closer can access variables outside of the body unlike functions
    let mut sample1 = 5;
    let print_var = || println!("sample1 = {}", sample1);
    print_var();
    sample1 = 10;
    // we can change value if we mark a closer with mut variable 
    let mut change_var = || sample1 +=1;
    change_var();
    println!("sample1 = {}", sample1);
    sample1 = 10;
    println!("sample1 = {}", sample1);

    // we can pass closer to functions
    fn use_func<T>(a: i32, b: i32 , func: T) -> i32
    where T: Fn(i32, i32) -> i32{
        func(a, b)
    }
    let sum = |a ,b| a+b;
    let prod  = |a ,b| a+b;
    println!("5 + 4 = {}", use_func(5, 4, sum));
    println!("5 * 4 = {}", use_func(5, 4, prod));

    // smart pointers
    // a pointer is an address to a location in a memory
    // they could be use to track the ownership of the data


    // Stack : stores values in an last in first out format
    // data on the stack must have a defined fixed size

    let b_int1 = Box::new(10);
    println!("b_int1 = {}", b_int1);

    struct TreeNode<T> {
        pub left: Option<Box<TreeNode<T>>>,
        pub right: Option<Box<TreeNode<T>>>,
        pub key: T,
    }
    impl<T> TreeNode<T>{
        pub fn new(key:T) -> Self {
            TreeNode { left: None, right: None, key,
            }
        }
        pub fn left(mut self, node: TreeNode<T>) -> 
        Self{
            self.left = Some(Box::new(node));
            self
        }
        pub fn right(mut self, node: TreeNode<T>) -> 
        Self{
            self.right = Some(Box::new(node));
            self
        }
    }
    let node1 = TreeNode::new(1).left(TreeNode::new(2)).right(TreeNode::new(3));


    // concurrency

    // common problems with parallel programming involve:
    // 1. Threads are accessing data in the wrong order
    // 2.threads are blocked form executing because of confusion
    // over requirements to proceed with execution 
    let thread1 = thread::spawn(||{
        for i in 1..25{
            println!("spawned thread {}",i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..20{
        println!("Main thread : {}", i );
    }
    thread1.join().unwrap();

    // real world example
    pub struct Bank {
        balance: f32
    }
    fn withdraw(the_bank: &Arc<Mutex<Bank>>, amt :f32){
        let mut bank_ref = the_bank.lock().unwrap();
        if bank_ref.balance < 5.00 {
            println!("current Balance : {} withdrawal a smaller amount",
        bank_ref.balance);
        }else{
            bank_ref.balance -= amt;
            print!("customer withdrew {} Current balance {} ",amt, bank_ref.balance);
        }
    }

    fn customer(the_bank: &Arc<Mutex<Bank>>){
        withdraw(&the_bank, 5.00);
    }
    let bank: Arc<Mutex<Bank>> = Arc::new(Mutex::new(Bank{balance: 20.00}));

    let handles = (0..10).map(|_| {
        let bank_ref = bank.clone();
        thread::spawn(|| {
            customer(bank_ref)
        })
    });
    for handle in handles{
        handle.join().unwrap();
    }
    println!("total {}", bank.lock().unwrap().balance);
}

// function

// fn get_some(x:i32,y:i32){
//     println!("{} + {} = {}", x, y, x + y);
// }

// function (if we need to return and value)

fn get_some_2(x: i32, y: i32) -> i32 {
    x + y
}

//function (return two value from 1 input)
fn get_some_3(x: i32) -> (i32, i32) {
    return (x + 1, x + 2);
}

fn sum_list(list: &[i32]) -> i32 {
    let mut sum = 0;
    for &val in list.iter() {
        sum += &val;
    }
    sum
}

// generic function
// we have to use this

use std::ops::Add;
use std::process::Output;

use crate::modules::order_food;

fn get_sum_gen<T: Add<Output = T>>(x: T, y: T) -> T {
    // we cant just return x + y, this add operator can not be used in this generic function
    return x + y;
}

// the concept of ownership

// stack : stores values in a last in first out format
// Data on the stack must have a defined fixed size

// HEAP : when putting data on the heap you request a certain amount of space.
// The OS finds space available and returns an address for that space called pointer

// RULES
// 1. Each values has a variable that's called it's owner
// 2. There is only one owner at a time
// 3. When the owner goes out of scope that value disappears

fn owner_ship() {
    let str_1 = String::from("world");
    let str_2 = str_1.clone();
    println!("Hello {}", str_2);
    let str_3 = str_1; // we copy str_1 into str_2, so str_1 does don't exists anymore
    println!("Hello {}", str_3);
    // println!("Hello {}", str_1) this will not work because position of codes matters, u assigner str_3 = str_1
}

fn print_str(x: String) {
    println!("A string {}", x);
}

fn print_return_str(x: String) -> String {
    println!("A string {}", x);
    x
}

fn change_string(name: &mut String) {
    name.push_str("is happy");
    println!("message : {}", name);
}
