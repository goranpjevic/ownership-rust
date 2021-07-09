fn main() {
    // copying values
    let first_integer = 10;
    let second_integer = first_integer; // the value of first_integer is copied into second_integer
    println!("The first integer is: {}\nThe second integer is: {}\n", first_integer, second_integer);

    // moving ownership into functions
    let first_string = String::from("This is the first String.");    // the value of the String is stored on the heap
    print_string(first_string); // the value of first_string is moved into the parameter of the function
    // the value of first_string and the variable itself is no longer valid here
    // this code will not compile:
    //  println!("{}", first_string);
    
    // moving ownership to new variables.
    let second_string = String::from("The ownership of this String will be moved.");
    let third_string = second_string;
    // the ownership of the value of second_string has been moved to third_string and so
    // second_string is no longer valid.
    // If we add this line, the code will not compile:
    //  println!("{}", second_string);
    println!("{}", third_string);

    // moving ownership into functions and getting the value back
    let new_string = String::from("This is a new String.");
    let returned_string = print_and_return(new_string);
    // new_string is no longer valid, but the value is now in returned_string
    println!("{}", returned_string);

    // referencing
    let mut string_to_be_referenced = String::from("This String will be referenced");
    print_string_by_reference(&string_to_be_referenced);
    // the value of string_to_be_referenced is borrowed, but the parameter in the function does not have
    // ownership of the value. Once the function ends, this value has not been cleared, since the
    // referenced_string in the print_string_by_reference() function does not own the value.
    println!("{}", string_to_be_referenced);

    // multiple references
    // we can either have immultiple mutable references or just one mutable one, withing the same
    // scope. That way we prevent data races.
    let first_immutable_reference = &string_to_be_referenced;
    let second_immutable_reference = &string_to_be_referenced;
    // if we create a mutable reference here, the code will not compile:
    //  let first_mutable_reference = &mut string_to_be_referenced;
    println!("{} {}", first_immutable_reference, second_immutable_reference);
    // first_immutable_reference and second_immutable_reference are no longer used and so have gone
    // out of scope and are no longer valid.
    // Now we can create a new mutable reference, because we cannot have data races:
    let second_mutable_reference = &mut string_to_be_referenced;
    println!("{}", second_mutable_reference);
}

fn print_string(some_string: String) {
    println!("{}", some_string);
}

fn print_and_return(some_string: String) -> String {
    println!("{}", some_string);

    some_string
}

fn print_string_by_reference(referenced_string: &String) {
    println!("{}", referenced_string);
}
