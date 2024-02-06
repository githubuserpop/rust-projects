//fn main() {
//    let x = 5;
//    println!("The value of x is: {x}");
//    x = 6;
//    println!("The value of x is: {x}");
//}
//^^^ This will not compile because x is immutable by default

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

// output: 
// The value of x is: 5
// The value of x is: 6
// This compiles because x is mutable now due to 'let mut x'