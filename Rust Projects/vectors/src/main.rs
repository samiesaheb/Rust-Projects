fn main() {
    /*
    //let a: Vec<i32> = Vec::new();
    //let v1: Vec<i32> = vec![1,2,3];

    // Can initialize a vector without type annotation!
    // Rust can infer the type by the values inserted into
    // the vector

    //println!("The first element is: {}", first)
    // for i in &v2 {
    //     println!("{}",i);
    // }

    // In order to increment each element of the vector, use below:
    // Using '*' - the dereference operator allows us to get to the
    // value in i before we can use the += operator*/
    let mut v2 = vec![5,6,7,8];

    let first = v2[0];

    v2.push(6);
    for i in &mut v2 {
        *i += 50;
    }
    println!("{:?}", v2);


    /*println!("{:?}", a);
    println!("{:?}", v1);
    println!("{:?}", v2);

    let mut v3 = Vec::new();

    v3.push(1);

    //println!("{:?}",v3);

    let third: i32 = v1[2];

    //println!("Third element of {:?} is {}", v1, third);

    match v1.get(2) {
        Some(third) => println!("Value of third element is {}", third),
        None => println!("There is no third element"),
    }*/

    // Can use an enum to store multiple types

    #[derive(Debug)]
    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Str(String),
    }

    // Defining a vector that holds the enum, ultimately the
    // different types
    let row = vec![
    SpreadSheetCell::Int(10),
    SpreadSheetCell::Float(3.14),
    SpreadSheetCell::Str(String::from("hello world!"))
    ];

    println!("{:?}", row);

}
