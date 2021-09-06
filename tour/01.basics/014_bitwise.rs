fn main(){
	let value = 0b11110011u8;
	println!("this is the int value of the binary {}", value);
	println!("value with 8 bit precision in case of binary {:8b}",value);

	println!(" the not operator ! of value {:8b} is {:8b}", value, !value);

	println!("bitwise and & of {:8b} & {:8b} is {:8b}", value, !value , value & !value);
	println!("common usecase of bitwise & is to find position 
		\n position of {:8b} at 6th place is", value & 0b010_0000);
	// likewise bitwise OR and bitwise XOR 
	// important to note that there is no carry over in bitwise operations


	// bit shift operators << - right shift, >> - left shift.
	// The values in the end or beginning or lost when they are shifted.
}