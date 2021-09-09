fn main() {
    let message = ['h', 'e', 'l', 'l', 'o'];


    // without enumerate we just get item
    // after adding enumerate to the iterator we also get index
    for (index, &item) in message.iter().enumerate() {
        println!("item {} is {}", index, item);
        if item == 'e' {
            break;
        }
    }

    for number in 10..5 {
        println!("number is {}", number);
    }

    // iterm_mut returns mutable values

     let mut matrix = [[1, 2, 3],
                      [4 ,5 ,6],
                      [7, 8, 9]];

    for row in matrix.iter_mut() {
        for num in row.iter_mut() {
           *num += 10;
           print!("{}\t", num);
        }
        println!();
    }
}
