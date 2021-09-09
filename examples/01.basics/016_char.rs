// in rust characters take 4 bytes. rust char supports unicode - which means emojis are included.


fn main() {
    let letter = 'a';
    let number = '1';
    let finger = '\u{261D}'; // to specify unicode append with u
    println!("{}\n{}\n{}", letter, number, finger);
}
