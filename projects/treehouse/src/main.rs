fn main() {
    println!("Hello, what's your name?");
    let mut your_name = String::new();
    use std::io::stdin;
    stdin().read_line(&mut your_name).expect("Failed to read line");
    println!("Hello, {}", your_name)
}
