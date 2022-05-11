fn main() {
	// This type of string can be mutated
	//let s1 = String::from("hello world"); 
	let s_1 = "what the fudge";
	//let mut str
	/*
	// Cannot have a seoncd mutable reference to a specific data
	{
	// Have to use curly braces to create new scope, which allows for
	// Multiple mutable references	
	let s3 = &mut s1; 

	}
	// let s2 = s1 // moves s1 into s2, cannot access s1 anymore after doing this
	//let s2 = s1
	

	let s2 = change(&mut s1); 

	let hello = &s2[0..5];
	let world = &s2[6..11];

	println!("{} {}",hello,world);

	// integers are stored in stack because size known at compile time
	let x = 5;
	let y = x;
	println!("x is {}, y is {}",x,y);
	*/
	//let word = first_word(&s1);
	let word2 = first_word(&s_1[..]);

	//s1.clear();

	println!("First word is {}", &word2);

	//println!("first word in s is: {}", ss);
}
/*
fn change(string: &mut String ) -> &mut String {

	// .push_str() appends a literal to a String
	string.push_str(" world");

	string
}*/

fn first_word(string: &str) -> &str {

	let bytes = string.as_bytes();
	for (i, &item) in bytes.iter().enumerate() {
		if item == 32 {
			return &string[..i];
		}
	} 
	&string[..]


}