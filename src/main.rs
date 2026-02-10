// fn main() {
//     let x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }
// // Problem because imutable variable is used
// // and we try to change it


// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }
// // Here the value of x can be changed because it's a let mut x


// fn main() {
//     const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
//     println!("{}", THREE_HOURS_IN_SECONDS)
// }
// // Constant always in uppercase with underscores between words


// // SHADOWING
// fn main() {
//     let x = 5;

//     let x = x + 1;

//     {
//         let x = x * 2;
//         println!("The value of x in the inner scope is: {x}");
//     }

//     println!("The value of x is: {x}");


//     let spaces = "          ";
//     let spaces = spaces.len();

//     println!("Number of spaces: {}", spaces)
// }
