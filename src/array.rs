use std::io;

pub fn arr(){
    let a = [11,22,33,44,55];

    println!("Please enter a array index");

    let mut index = String::new();
    
    io::stdin().read_line(&mut index).expect("failed to read line");

    let index: usize = index.trim().parse().expect("Please enter a number");

    let element = a[index];

    println!("The value of index is {index} is {element}");
}