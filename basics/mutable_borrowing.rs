fn main(){
    // prints right tick green colored mark emoji
    println!("\u{2705} Right tick green colored mark emoji");
    // prints red cross mark emoji
    println!("\u{274C} Red cross mark emoji");

    // correct: mutable borrowing
    println!("\u{1F4A1} Correct: Mutable borrowing");
    let mut x = 10;
    increment(&mut x);
    println!("Value after increment: {}", x);

    // incorrect: borrowing mutable from immutable reference
    println!("\u{1F4A1} Incorrect: Borrowing mutable from immutable reference");
    let y = 20;
    // Uncommenting the next line will cause a compilation error
    // increment(&mut y); // This line is incorrect because `y` is not mutable
    // fix: change `y` to mutable
    let mut y = y;
    increment(&mut y);
    // y after fix
    println!("Value of y after fix: {}", y);

    // error: mutable and immutable borrow at the same time
    println!("\u{1F4A1} Error: Mutable and immutable borrow at the same time");
    let mut z = 30;
    let r1 = &z; // immutable borrow
    // Uncommenting the next line will cause a compilation error
    increment(&mut z); // This line is incorrect because `z` is already borrowed
    // fix: use a separate scope for mutable borrow
    
    let mut z = 30;
    {
        let r2 = & mut z; // immutable borrow
        *r2 += 5; // mutable borrow in a separate scope
    }
    println!("Value of z after fix: {}", z);


    // print line break with underscores
    println!("\n__________________________________________________________\n");
    // Immutable borrowing, safe to share
    let x = 41;
    let r1 = &x; // immutable borrow
    let r2 = &x; // another immutable borrow
    // print okay with correct emoji
    println!("\u{1F4A1} Original value: x: {}", x);
    println!("\u{1F4A1} Immutable borrowing, safe to share: x {} r1 {} r2 {}", x, r1, r2);
    println!("\u{2705} Immutable borrowing, safe to share: r1 {}", r1);
    println!("\u{2705} Immutable borrowing, safe to share: r2 {}", r2);
    println!("\n__________________________________________________________\n");

    // Mutable borrowing, exclusive access
    let mut y = 42;
    println!("\u{1F4A1} Original value: y: {}", y);
    let r3 = &mut y; // mutable borrow
    *r3 += 1;
    println!("\u{2705} Mutable borrowing, exclusive access: r3 {}", r3);
    // Uncommenting the next line will cause a compilation error
    // let r2 = &y; // This line is incorrect because `y` is already mutably borrowed

    // also we cannot use y because it is already mutably borrowed and mutable borrow is exclusive
    // fix: use a separate scope for mutable borrow
    let mut y = 42;
    {
        let r4 = &mut y; // mutable borrow in a separate scope
        *r4 += 1; // increment the value
        println!("\u{2705} Mutable borrowing in a separate scope: r4 {}", r4);
        // r4 goes out of scope here, or we say it is dropped, so we can use y again
        println!("\u{2705} Mutable borrow r4 is dropped, we can use y again");
    }
    // now we can use y again
    println!("\u{2705} Value of y after fix: {}", y);
    println!("\n__________________________________________________________\n");

    let mut x = 10;
    let r = &mut x; // mutable borrow
    println!("\u{1F4A1} Original value: x: {}", x);
    *r += 1; // increment the value through the mutable reference
    println!("\u{2705} Value of x after increment through mutable reference: {}", x);



}

fn increment(value: &mut i32) {
    *value += 1; // dereference and increment the value
}