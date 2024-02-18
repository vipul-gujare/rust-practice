use std::io::stdin;

fn main() {
    let mut name: String = String::new();
    println!("Who is this? ");
    let _ = stdin().read_line(&mut name);

    println!("Hello {}", name);
}
