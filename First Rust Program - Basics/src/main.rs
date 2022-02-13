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

}
