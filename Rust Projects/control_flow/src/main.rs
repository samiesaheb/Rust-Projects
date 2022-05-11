use std::io;

fn main() {

    loop {

		println!("Enter a number:");

	    let mut x = String::new();
	    
	    io::stdin()
	    	.read_line(&mut x)
	    	.expect("Could not readline!");

	   	let x: i32 = x.trim().
	   					 parse().
	   					 expect("Did not enter a number!");

	    let checkif = check(x);

	    println!("{}\n---------------", checkif);

	    if checkif == "Equal to 5!" {

	    	break;

    	}
	}

	let res = counter_practice();

	println!("The result is {}", res);

	let a = [1,2,3,4,5];
	print_array(&a);

	let mut n = String::new();


	println!("Write down the number for range:");

	io::stdin()
	    	.read_line(&mut n)
	    	.expect("Could not readline!");


   	let n: i32 = n.trim().
   				   parse().
   				   expect("Did not enter a number!");


   	println!("Printing the range!");

   	range(n);

    //let val = check_plus_five(checkif);

    //println!("value is now {}", val);

}

fn check(n: i32) -> &'static str /*used to return str*/ {

	if n > 5 {

		return "Greater than 5!"

	}
	if n == 5 {

		return "Equal to 5!"

	}
	else {
		return "Smaller than 5!"

	}
}

fn counter_practice() -> i32 {

	let mut i = 0;

	let result = loop {

		i += 1;

	if i == 10 {

		break i*2;

		}
	};

	return result
}

fn print_array(array: &[i32]) {

	//let mut index = 0;
	//let len_of_array = array.len();

	/*while index < len_of_array {

		println!("{} {}", index ,array[index]);
		index += 1;

	}*/

	// Printing elements of array using for loop
	for e in array.iter() {

		println!("{}",e);

	}

}

fn range(n: i32) {

	for i in (1..=n).rev().rev() {

		println!("{}",i);

	}

}








