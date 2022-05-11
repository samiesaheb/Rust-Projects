fn main() {

    let arr = function_one();

    let ten = function_two();

    println!("function 1 returns {:?}", arr);

    println!("function 2 returns {}", ten);


}


fn function_one() -> [i32;5] {

	let a = [1,2,3,4,5];

	return a;
}

fn function_two() -> i32 {

	let y = {
		let x = 9;
		x + 1
	};

	return y;
}