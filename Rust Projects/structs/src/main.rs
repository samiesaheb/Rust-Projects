fn main() {


    let samie = User {

    	firstname: String::from("Samie"),
    	lastname: String::from("Saheb"),
    	age: 22,
    	active: true,
    };

    let george_fname = String::from("George");
    let george_lname = String::from("Hotz");
    let george = create_user(george_fname,george_lname,31);

    let lex = User {
    	firstname: String::from("Lex"),
    	lastname: String::from("Fridman"),
    	age: 33,
    	..samie
    };

    println!("Users:\n");
    println!("Name is {} {}\n{} years old\nActive: {}",
    	samie.firstname, samie.lastname, samie.age, samie.active);

    println!("\nName is {} {}\n{} years old\nActive: {}",
    	george.firstname, george.lastname, george.age, george.active);

	println!("\nName is {} {}\n{} years old\nActive: {}",
	    	lex.firstname, lex.lastname, lex.age, lex.active);

	struct Color(i32, i32, i32);
	struct Point(i32, i32, i32);

	let red = Color(255,0,0);
	let point = Point(1,0,1);

	println!("Colour is {}", red);

}


struct User {
	firstname: String, 
	lastname: String,
	age: i32,
	active: bool,
}

fn create_user(firstname: String, lastname: String, age: i32) -> User {

	User {
		firstname,
		lastname,
		age,
		active: true,
	}
}