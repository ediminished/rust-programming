fn main(){
    // if else
    let number = 5;
    if number < 10 {
        println!("The number is less than 10");
    } else {
        println!("The number is 10 or greater");
    }
    let number = 15;
    // only if
    if number < 10 {
        println!("The number is less than 10 (won't  be printed)");
    }
    // if with multiple conditions
    if number < 10 {
        println!("The number is less than 10");
    } else if number < 20 {
        println!("The number is less than 20");
    } else {
        println!("The number is 20 or greater");
    }

    // boolean expressions
    let is_even = number % 2 == 0;
    println!("Is the number {} even? {}",number, is_even);
    if is_even {
        println!("The number is even");
    } else {
        println!("The number is odd");
    }
    // this won't compile as 1 is not a boolean
    // if 1 {
    //     println!("This condition is always true, as 1 is truthy in Rust");
    // }
    // if can return a value
    let result = if is_even {
        "EVEN"
    } else {
        "ODD"
    };
    println!("The number {} is {}", number, result);

    // match statement
    let code = 404;
    match code {
        200 => println!("OK"),
        404 => println!("Not Found"),
        500 => println!("Internal Server Error"),
        _ => println!("Unknown Error"), // _ is a catch-all pattern
    }
    // match with multiple patterns
    let status = "success";
    match status {
        "success" | "ok" => println!("Operation was successful"),
        "error" => println!("An error occurred"),
        _ => println!("Unknown status"),
    }

    // loop
    let mut count = 0;
    loop {
        count += 1;
        println!("Count is {}", count);
        if count == 5 {
            break; // exit the loop when count reaches 5
        }
    }
    // return value from loop
    let result = loop {
        count += 1;
        if count == 10 {
            break count * 2; // return double the count when it reaches 10
        }
    };
    println!("The result from the loop is {}", result);

    // for loop
    for i in 1..=5 { // inclusive range
        println!("For loop iteration: {}", i);
    }

}