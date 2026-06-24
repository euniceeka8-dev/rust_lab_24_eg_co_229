fn take_ownership(s: String) {
    println!("{}", s);
}

fn print_from_main(t: &String) {
    println!("{}", t);
}

fn main() {
    let x = 5;
    let y = 10;
    let z = 15;
    let s = String::from("Hello");
    let t = String::from("World");

    take_ownership(s);       // ownership of s is moved here
    print_from_main(&t);     // borrow t by reference

    println!("x: {}", x);
    println!("y: {}", y);
    println!("z: {}", z);
}
