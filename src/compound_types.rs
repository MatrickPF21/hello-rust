pub fn main() {
    // Declare array, initialize all values to 0, length = 5
    let bytes = [0; 5];

    // Declare array, initialize all values, compiler infers length = 7
    // Days of the week
    let days = [
        "Sunday",
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
    ];

    // Get the first day of the week
    let first = days[0];

    // Get the second day of the week
    let second = days[1];

    // Get the seventh day of the week using the wrong index - should be 6
    // Compiler proceeds to yell at you
    // let seventh = days[7];

    // Declare vector, initialize with three values
    let three_nums = vec![15, 3, 46];
    println!("Initial vector: {:?}", three_nums);

    // Declare vector, value = "0", length = 5
    let zeroes = vec![0; 5];
    println!("Zeroes: {:?}", zeroes);

    // Create empty vector, declare vector mutable so it can grow and shrink
    let mut fruit = Vec::new();

    // Push values onto end of vector, type changes from generic `T` to String
    fruit.push("Apple");
    fruit.push("Banana");
    fruit.push("Cherry");
    println!("Fruits: {:?}", fruit);

    // Push integer value, but vector expects String (&str) type value
    // Compiler proceeds to yell at you
    // fruit.push(1);

    // Pop off value at end of vector
    // Call pop() method from inside println! macro
    println!("Pop off: {:?}", fruit.pop());
    println!("Fruits: {:?}", fruit);

    // Declare vector, initialize with three values
    let mut index_vec = vec![15, 3, 46];
    let three = index_vec[1];
    println!("Vector: {:?}, three = {}", index_vec, three);

    // Add 5 to the value at index 1, which is 5 + 3 = 8
    index_vec[1] = index_vec[1] + 5;
    println!("Vector: {:?}", index_vec);

    println!("Vector length: {}", index_vec.len());

    // Access vector with out-of-bounds index
	// The program aborts with an error message
    // let beyond = index_vec[10];
    // println!("{}", beyond);
}
