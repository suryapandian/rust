//Generic types allow us to partially define a struct or enum
// A partially defined struct type
struct BagOfHolding<T> {
    item: T,
}

#[derive(Debug)]
struct Rectangle<T, U> {
    width: T,
    height: U,
}

impl<T, U> Rectangle<T, U> {
    fn get_width(&self) -> &T {
        &self.width
    }
}

/*
Rust generally can infer the final type by looking at our instantiation, but if it needs help you can always be explicit using the ::<T> operator, also known by the name turbofish
*/
fn main() {
    // Note: by using generic types here, we create compile-time created types.
    // Turbofish lets us be explicit.
    let i32_bag = BagOfHolding::<i32> { item: 42 };
    let bool_bag = BagOfHolding::<bool> { item: true };

    // Rust can infer types for generics too!
    let float_bag = BagOfHolding { item: 3.14 };

    // Note: never put a bag of holding in a bag of holding in real life
    let bag_in_bag = BagOfHolding {
        item: BagOfHolding { item: "boom!" },
    };

    println!(
        "{} {} {} {}",
        i32_bag.item, bool_bag.item, float_bag.item, bag_in_bag.item.item
    );

    let rect = Rectangle {
        width: 1u8,
        height: 3u16,
    };
    println!("rect is {:?}", rect);
    println!("width is {}", rect.get_width());
}
