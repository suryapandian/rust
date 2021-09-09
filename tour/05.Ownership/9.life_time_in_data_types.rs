// Now this lifetime annotation does not do anything to affect how long the references it gets passed as inputs lift. 
// That's purely on those referenced variables themselves. 
// What it does is tell the compiler how the lifetimes of the input parameters relate to each other.


fn best_fuel<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let result;
    let propellant1 = String::from("RP-1");
    {
        let propellant2 = String::from("LNG");
        result = best_fuel(&propellant1, &propellant2);
    }
    println!("result is {}", result);
}
