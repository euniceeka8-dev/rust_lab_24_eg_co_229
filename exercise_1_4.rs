fn take_ownership(s: String) -> String {
    s
}

fn borrow_ownership(s: &String) -> &String {
    s
}

fn main() {
    let s = String::from("hello");
    let s1 = take_ownership(s);       // ownership of s is moved into s1
    let s2 = borrow_ownership(&s1);   // borrow s1 by reference
    println!("{}", s2);
}
