fn main() {
    let first_integer = 10;
    let second_integer = first_integer; // the value of first_integer is copied into second_integer
    println!("The first integer is: {}\nThe second integer is: {}\n", first_integer, second_integer);

    let first_string = String::from("This is the first String.");    // the value of the String is stored on the heap
    print_string(first_string); // the value of first_string is moved into the parameter of the function

    // the value of some_string and the variable itself is no longer valid here
    // this code will not compile:
    //  println!("{}", some_string);

    let new_string = String::from("This is a new String.");
    let returned_string = print_and_return(new_string);
    // new_string is no longer valid, but the value is now in returned_string
    println!("{}", returned_string);
}

fn print_string(some_string: String) {
    println!("{}", some_string);
}

fn print_and_return(some_string: String) -> String {
    println!("{}", some_string);

    some_string
}
