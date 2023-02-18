fn main() {
    let message: String = hello("world");

    println!("{message}");
}

fn hello(target: &str) -> String {
    let res = format!("Hello, {target}!");

    res
}
