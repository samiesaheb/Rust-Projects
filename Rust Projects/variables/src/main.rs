fn main() {
    
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    const MAX_POINTS: u32 = 100000;
    // const MY_NAME: &str = "Samie"; //Creating a const string using '&str = '
    println!("Max points is {}", MAX_POINTS);

    let spaces = "   ";
    let spaces  = spaces.len();
    println!("Number of spaces is {}", spaces);
}
