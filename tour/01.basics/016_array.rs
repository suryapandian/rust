fn main() {
    // in rust array size should be defined before compile time
    /* if we try to access an array value that is not defined, 
    rust does not return default value
    rust throws compiler error
    */
    let mut letters = ['a', 'b', 'c'];
    letters[0] = 'x';
    let first_letter = letters[0];
    println!("first_letter is {}", first_letter);

    let numbers: [i32; 5];
    numbers = [0; 5];
    let index: usize = numbers.len();
    println!("last number is {}", numbers[index]);

    //the size of each array in multidimensional array should be same to avoid compile time errors
}