struct MouseClick {
    x: i64,
    y: i64,
}

fn main() {
	let tuple = ("hello", 10, -10, true, (1, 2, 3));
    let (first, second, third, fourth, _) = tuple;

    println!("First element is: {}", first);
    println!("Second element is: {}", second);
    println!("Third element is: {}", third);
    println!("Fourth element is: {}", fourth);

    // Addition, Subtraction, and Multiplication
    println!(
        "1 + 2 = {} and 8 - 5 = {} and 15 * 3 = {}",
        1u32 + 2,
        8i32 - 5,
        15 * 3
    );

    // Integer and Floating point division
    println!("9 / 2 = {} but 9.0 / 2.0 = {}", 9u32 / 2, 9.0 / 2.0);

    struct Color(i32, i32, i32);
    struct Unit;

    let myColor = Color(10, 20, 30);
    let myUnit = Unit;

    let myarr = [1, 2, 3, 4, 5];

    for num in myarr {
        println!("{}", num);
    }
    for num in (1..4).rev() {
        println!("asD: {num}!");
    }

    // let guess: bool = "42".parse().expect("Not a boolean!");
    // let bigInt: i16 = 100000000000000000000000000000000000000000
    // addition
    let sum = 5 + 10;
    println!("{}", sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("{}", difference);

    // multiplication
    let product = 4 * 30;
    println!("{}", product);

    // division
    let quotient = 56.7 / 32.2;
    println!("{}", quotient);
    let truncated = -5 / 3; // Results in -1
    println!("{}", truncated);

    // remainder
    let remainder = 43 % 5;
    println!("{}", remainder);

    let arr = [MouseClick { x: 1, y: 2 }, MouseClick { x: 4, y: 5 }];

    let myPoint = MouseClick { x: 10, y: 20 };

    let MouseClick { x, y } = myPoint;

    println!("x = {}, y = {}", x, y);
}