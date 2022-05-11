fn main() {

    let one = String::from("one");
    let two = String::from("two");
    let three = String::from("three");

    // c.push_str("c");
    // c.push('c'); //pushes char, HAVE to use 'c' which != "c", to the variable c
    // println!("{}",c);

    let onetwothree = format!("{}-{}-{}", one, two, three);

    println!("{}",onetwothree);
    // println!("{}-{}-{}",one,two,three);

    //let s = String::from("hello");
    // let h = s[0]; <- Does not work, Rust does not support string indexing.

    let hello = String::from("hello");

    // Can take string slices though:
    let h = &hello[..1];
    println!("First letter in hello is {}",h);

    // Iterating over strings to access each element using .char() method on string,
    // Can use .bytes() to access the elements raw byte value
    for i in hello.chars() {
        println!("{}", i);
    }
}
