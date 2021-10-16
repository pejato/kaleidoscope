pub mod tokenization;

fn main() {
    let my_str = "1.23.4".to_string();

    println!("{}", my_str.parse::<f64>().unwrap());
    println!("Hello, world!");
}
