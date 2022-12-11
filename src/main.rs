#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write,BufRead,BufReader,ErrorKind};
use std::fs::File;
use std::cmp::Ordering;


fn main() {
    println!("What is your Name?");
    let mut name: String = String::new();
    let greeting: &str = "nice to meet you";
    io::stdin().read_line(&mut name)
        .expect("didn't receive input");
    println!("Hello {}! {}", name.trim_end(), greeting);
    // another type of variable 
    const ONE_MIL: u32 = 1_000_000;
    println!("{}",ONE_MIL );
    // how can we define float
    const PI: f32 = 3.141;
    // we need to define char with single quotes and string with double quotes
    let age: &str = "42";
    // we can define variable with the same name with different data types
    // here u32 means unsigned 32 bit 
    let mut age: u32 = age.trim().parse()
        .expect("age wasn't assigned as number");
    age = age + 1;
    println!("i am {} and i want $ {}", age, ONE_MIL );

    // data types
    // unsigned integer : u8, u16, u32, u64, u128, usize
    // singed integer : i8, i16, i32, i64, i128, isize
    println!("Max u32 : {}",u32::MAX);
    println!("Max u64 : {}",u64::MAX);
    println!("Max usize : {}",usize::MAX);
    println!("Max u128 : {}",u128::MAX);
    println!("Max f32 : {}",f32::MAX);
    println!("Max f64 : {}",f64::MAX);

    // another data type is booleans
    let is_true: bool = true;
    let my_grade="A";

    // math
    let num_1: f32 = 1.111111111111111;
    println!("f32:{}", num_1 + 0.111111111111111);
    let num_2: f32 = 1.111111111111111;
    println!("f64:{}", num_2 + 0.111111111111111);
    let mut num_3: u32 = 5;
    let num_4: u32 = 4;
    println!("5 + 4 : {}",num_3 + num_4);
    println!("5 - 4 : {}",num_3 - num_4);
    println!("5 / 4 : {}",num_3 / num_4);
    println!("5 * 4 : {}",num_3 * num_4);
    println!("5 % 4 : {}",num_3 % num_4);
    num_3 += 1;

    // generate random values
    let random_num = rand::thread_rng().gen_range(1..101);
    println!("random : {}", random_num);
    
    // if statement 
    let age2: i32 = 8;
    if (age2 >= 1) && (age2 <= 18){
        println!("important birthday");
    } else if(age2 == 21 ) || (age2 == 50){
        println!("IMPORTANT BIRTHDAY ");
    }else if age2 >= 65{
        println!("Important birthday");
    } else {
        println!("moira jan bro");
    }
    // ternary operator 
    let mut my_age = 47;
    let can_vote = if my_age >= 18{
        true
    }else{
        false
    };
    println!{"can vote : {} ", can_vote}
    // match
    let age3 = 8;
    match age3 {
        1..= 18 => println!("Important birthday"),
        21 | 50 => println!("IMPORTANT BIRTHDAY"),
        65..=i32::MAX => println!("Important Birthday"),
        // _ means = match everything else 
        _ => println!("Not an important birthday"),
    };
    // another example of match
    let my_age2=18;
    let voting_age2 =18;
    match my_age2.cmp(&voting_age2){
        Ordering::Less => println!("you gained the right to vote"),
        Ordering::Greater => println!("you don't have the permission to vote"),
        Ordering::Equal => println!("you gained the right to vote"),
    }

    // arrays
    // elements of an array must be of same type 
    // and a array has the same size
    let arr_1 = [1,2,3,4,5,6,7];
    println!("1st : {}", arr_1[01]);
    println!("1st : {}", arr_1.len());
    let arr_2 = [1,2,3,4,5];
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
    let my_tuple:(u8,String,f64) = (47, "Nahiyan".to_string(),50_000.00);
    println!("Name {}",my_tuple.1);
    // assign multiple tuples
    let (v1,v2,v3) =  my_tuple;
    println!("Age : {}",v1);

    // strings
    // there are two types of string
    // String  --> vector of bytes that can be change 
    // &str  --> and string type which is going to point to the string
    // and allow for viewing of said string
    let  mut st1 = String::new();
    st1.push('A');
    st1.push_str("words");
    for word in st1.split_whitespace(){
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
    for char in v1{
        println!("{}",char);
    }
    let st4: &str = "Random string";
    let mut st5: String = st4.to_string();
    println!("{}",st5);
    let byte_arr1 = st5.as_bytes();
    let st6 = &st5[0..6];
    println!("string length {}", st6.len());
    st5.clear();
    // combine string
    let st6 = String::from("just some");
    let st7 = String::from("words");
    let st8 = st6 + &st7;
    for char in st8.bytes(){
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
    Sunday
    }
// . is used when you have a value on the left-hand-side. :: is used when you have a type or module. Or: . is for value member access, :: is for namespace member access.
    impl DayName {
        fn is_weekend(&self) -> bool {
            match self {
                DayName::Saturday | DayName::Sunday => true,
                _ => false
            }
        }
    }
    let today = DayName::Monday;
    match today{ 
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
    let mut vec2 = vec![1,2,3,4];
    vec2.push(5);
    // print from specific index
    println!("1st : {}", vec2[0]);
    // verify value exists 
    let second: &i32 = &vec2[1];
    match vec2.get(1){
        Some(second) => println!("2nd : {}",second),
        None => println!("No 2nd value")
    }
    // we can cycle and can change value 
    for i in &mut vec2{
        *i *= 2;
    }
    for i in &vec2{
        println!("{}",i);
    }
    println!("Vec length {}", vec2.len());
    println!("Pop : {:?}", vec2.pop());
    
    // function called here
    // get_some(5,4);
    println!("{}", get_some_2(5, 4));

    // get the two value from the function 
    let (val_1, val_2) = get_some_3(3);
    println!("Nums {} {} ", val_1,val_2);

    // function and list
    let num_list = vec![1,2,3,4,5];

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
fn get_some_3(x:i32) -> (i32,i32) {
    return (x+1 , x+2);
}

fn sum_list(x:i32) -> (i32,i32) {
    return (x+1 , x+2);
}