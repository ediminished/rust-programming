fn main() {
    // mutable borrowing exercises
    println!("\u{1F4A1} Mutable borrowing exercises");
    let mut z = 30;
    let m1 = &mut z; // mutable borrow
    *m1 += 5; // increment the value through the mutable reference
    println!("\u{2705} Value of z after increment through mutable reference: {}", z);
    // info emojis
    println!("\u{1F4A1} Original value: z: {}", z);
    println!("Feel free to use z again after this point, as the mutable borrow is no longer in use.");
    println!("\n__________________________________________________________\n");
    // Mutable borrowing, exclusive access
    let mut y = 42;
    println!("\u{1F4A1} Original value: y: {}", y);
    // Immumtable borrow from y which is mutable
    let m1 = &mut y; // immutable borrow
    println!("\u{2705} Immutable borrowing, safe to share: m1 {}", m1);
    // Uncommenting the next line will cause a compilation error
    let m2 = &mut y; // This line will be incorrect if m1 is used later
    println!("\u{2705} Immutable borrowing, safe to share: m2 {}", m2);

    println!("\u{1F4A1} Immutable borrow m2 is not allowed, as y is already borrowed immutably {}", m1);

}