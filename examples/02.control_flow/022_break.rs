fn main() {
    let mut count = 0;

    let result = loop {
        if count == 10 {
            break count * 10; // we can return value after break
        }
        count += 1;
        println!("count is {}", count);
    };

    println!("After the loop!");
    println!("result is {}", result);
}


// loops are almost similar to while, except that while cannot return a value