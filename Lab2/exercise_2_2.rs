// Lab Session 2 - Control Flow & Pattern Matching
// Exercise 2.2: if/else and Loops

fn main() {
    // 1. if / else as an expression
    let number = 229; // derived from my reg number (24/EG/CO/229)
    let max = if number % 2 == 0 { "Even" } else { "Odd" };
    println!("{} is {}", number, max);

    // 2. loop with a break value
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("Loop result: {}", result);

    // 3. while loop
    let mut n = 3;
    while n != 0 {
        println!("{}!", n);
        n -= 1;
    }
    println!("Liftoff!");

    // 4. for loop over a range
    let sum: i32 = (1..=100).sum();
    println!("Sum 1..=100 = {}", sum);

    // TODO 1: Using a for loop and a range, print the multiplication
    // table for 229, from 229x1 through 229x12
    println!("\nMultiplication table for {}", number);
    for i in 1..=12 {
        println!("{} x {} = {}", number, i, number * i);
    }
}