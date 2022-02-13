extern crate rand;
use rand::{thread_rng, Rng};


fn main() {
    //Variables are immutable
    //Static type - once a type is declared or inferred it cant be changed
    let x="Rohit";
    //Variable with a type
    let number: u32 = 1024;
    //Mutable variable
    let mut y ="Rohit";
    //Declaring a tuple
    let tup= (420,"Hi Rohit",true);
    //Defining an array
    let arr=[1,2,3,4,5];
    //Declaring an array of zeros
    let arrz:[u8;5]=[0;5];
    println!("Hello {}",x);
    println!("Hello {}",y);
    y ="Chillar";
    println!("Hello {}",y);
    println!("Unsigned 32 bit {}",number);
    //Printing a element of a tuple
    println!("Party time {}",tup.0);
    //accessing values from a tuple - Destructuring
    let (val_one,val_two,_)=tup;
    println!("Party time {} {}",val_one,val_two);
    //printing array element
    println!("Array first element {}",arr[0]);
    //printing all elements
    println!("Array is {:?   }",arrz);
    //if else statement and loops
    let mut i=1;
    //while
    while arr[i]<3 {
        println!("While {} {}",i,arr[i]);
        i=i+1; 
    }
    //for
    for j in 1..=4{

        if arr[j]>4{
            println!("Break");
            break   
        }
        else if arr[j]>2 && arr[j]<4{
            println!("For else if {} {}",j,arr[j])
        }
        else{
            println!("For else {} {}",j,arr[j])
        }
    }
    //Match
    match x {
        "Rohit"=> println!("Value of x is {}",x),
        "Chillar"=> println!("Value of x is {}",x),
        _=>println!("Value of x is invalid")
    }

    let key:&str="abaa";
    let s:&str="bbbb";
    let c= key.chars().nth(2).unwrap();
    for byte in key.bytes() {

        println!("{}",byte);
        // ...
    }
    println!("{}",c);

    let speed:u8=5;
    let mut f:f64 = speed.into();

    f=f*(2 as f64);
    println!("convert {}",f)


}

///simple cypher problem on exercism

use std::ops::Range;
static RANGE: Range<u8> = b'a'..(b'z' + 1);

pub fn add(c: char,k: char) -> char{
    (b'a' + u8::wrapping_add(26 + c as u8 - b'a', k as u8 - b'a') % 26) as char       
}

pub fn sub(c: char,k: char) -> char{
    (b'a' + u8::wrapping_sub(26 + c as u8 - b'a', k as u8 - b'a') % 26) as char       
}

pub fn encode(key: &str, s: &str) -> Option<String> {
    if key.is_empty() || !key.chars().all(|c| c.is_ascii_lowercase()) {
            return None;
    }
    Some(s.chars()
        .zip(key.chars().cycle())
        .map(|(c, k)| add(c,k)).collect(),)
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    if key.is_empty() || !key.chars().all(|c| c.is_ascii_lowercase()) {
            return None;
    }
    Some(s.chars()
        .zip(key.chars().cycle())
        .map(|(c, k)| sub(c,k)).collect(),)
}

pub fn encode_random(s: &str) -> (String, String) {
    let mut rng = thread_rng();
    let key: String = (0..100).map(|_| (rng.gen_range(RANGE.start, RANGE.end)) as char).collect();
    let encoded_string = encode(&key, s);
    (key, encoded_string.unwrap())
}
