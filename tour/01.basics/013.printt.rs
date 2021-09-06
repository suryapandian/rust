fn main (){
	let var1 = 10.0;
	let var2 = 3.0;
	let result = var1/var2;
	println!("{}/{} = {} ", var1, var2, result);
	print!("prints without new line {}", var1);
	println!("this will print upto 3 precision {:.3}", result);
	println!("this will print with leading zeros  taking exactly 8 characters in total {:08.3}", result);

	println!("{1}#1 {0}#0 {}#1 ", var1, result);
}