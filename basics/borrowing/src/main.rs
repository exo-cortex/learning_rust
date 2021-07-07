fn main() {

	let s1 = String::from("hello!");
	let s2 = s1.clone(); // needs to be cloned
	
	println!("s1 = {}, s2 = {}", s1, s2);

	let x = 5;
	let y = x; // needs NOT to be cloned,because size is known (lives on the stack) at compile time.

	println!("x = {}, y = {}", x, y );

}