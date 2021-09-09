struct Color(u8, u8, u8); // RGB
struct Point(u8, u8, u8); // XYZ
struct Marker;
// for standard structures like co-ordinates it is well known that first is x, then y and z
// so it makes sense for us to use struct tuple instead of using regular struct

fn get_y(p: Point) -> u8 {
    p.1
}

fn main() {
    let red = Color(255, 0, 0);
    println!("First value is {}", red.0);

    let coord = Point(4, 5, 6);
    let y = get_y(coord);
    println!("y is {}", y);
     let _m = Marker; //unit like struct,  a unit is another word for an empty tuple (). This is why this kind of struct is called Unit-like.
}