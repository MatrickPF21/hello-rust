fn class() {
    let number = 3;

    // you cannot specify a non boolean value in an if statement

    // this will fail
    // if number {
    //     println!("number was three");
    // }

    // this will work
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // the inline if statement should return the same type in both branches
    let number = if number != 0 { 10 } else { 20 };

    // this will never end
    // loop {
    //     println!("again!");
    // }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn fahrenheit_to_celsius(f_grade: u16) -> u16 {
    return ((f_grade - 32) * 5) / 9;
}

fn fibo(n: u32) -> u32 {
    if n == 0 || n == 1 {
        return n;
    }

    return fibo(n - 1) + fibo(n - 2);
}

fn merry_xmas() {
    let xmas: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let xmas_verse: [&str; 12] = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a laying",
        "Seven swans a swimming",
        "Eight maids a milking",
        "Nine ladies dancing",
        "Ten lords a leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    let mut counter = 0;

    while counter < 12 {
        println!(
            "On the {} day of Christmas, my true love sent to me",
            xmas[counter]
        );

        let mut verse_counter = counter;

        for verse in (0..verse_counter + 1).rev() {
            println!("{}", xmas_verse[verse]);
        }

        counter += 1;
    }
}

fn main() {
    assert_eq!(fahrenheit_to_celsius(32), 0);
    assert_eq!(fahrenheit_to_celsius(212), 100);

    assert_eq!(fibo(0), 0);
    assert_eq!(fibo(1), 1);
    assert_eq!(fibo(2), 1);
    assert_eq!(fibo(3), 2);
    assert_eq!(fibo(4), 3);
    assert_eq!(fibo(5), 5);
    assert_eq!(fibo(6), 8);

    merry_xmas();
}
