use std::io;

fn main() {

    //let guess: u32 = "42".parse().expect("Not a number!");
    /*let sum = 5+10;
    println!("sum is {}", sum);

    let tup: (i32, f32, u32) = (1,1.0,1);
    let (x,y,z) = tup;

    println!("x is {}\ny is {}\nz is {}", x, y, z);

    let ex: (i32, f64, u8) = (200,10.0,11);

    let two_hundred = ex.0;
    let ten_point_zero = ex.1;
    let eleven = ex.2;

    println!("0th index element of x: {}\n1st index element of x: {}\n2nd index element of x: {}", 
    									   two_hundred,
    		  							   ten_point_zero,
    		  							   eleven);
    let arr: [f32;4] = [2.0, 1.0, 3.0, 4.0];

    let arr2: [f32;5] = [1.0;5];

	println!("{:?}", arr);

	println!("{:?}", arr2);*/

	let a = [1,2,3,4,5];

	println!("Please enter an array index!");

	let mut index = String::new();

	io::stdin()
		.read_line(&mut index)
		.expect("Failed to read line!");

	let index: usize = index
		.trim()
		.parse()
		.expect("Index entered was not a number!");

	let element = a[index];

	println!("The value of element at index {} of {:?} is {}",
			index, a, element);



}
