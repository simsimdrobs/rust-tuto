fn main() {
    play_with_strings();
    println!("Hello, world!");
}

fn play_with_strings() {
    let a = String::from("hello");
    let b: &str = "toto";
    println!("{} {}", a, b);
}