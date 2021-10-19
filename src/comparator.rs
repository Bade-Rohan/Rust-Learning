use std::io;
use std::cmp::Ordering;
pub fn compare(){
    println!("*********Program Comparator*******");
    //the following program helps to distinguish the age and give appropriate message
    let actual_age:i32 =15; //the guess is made to zero
    let mut guess = String::new();
    println!("Enter your age: ");
    io::stdin().read_line(&mut guess).expect("Error");
    let guess:i32 = guess.trim().parse().expect("Error");
    match guess.cmp(&actual_age){
        Ordering::Less =>println!("People below age 15 are not allowed"),
        Ordering::Greater|Ordering::Equal =>println!("Welcome !!! "),         
    }
    println!("*******End of program Comparator****");
}
