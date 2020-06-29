fn main() {
    use std::io::{stdin};
    let mut s = String::new();
    println!("Hello,\n\
        Would you like to play a game?\n\
        1. Yes\n\
        2. No");
    
    println!("\n\n--- Press enter to exit ---");
    stdin().read_line(&mut s).expect("Did not enter a string");
}