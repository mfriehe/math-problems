
fn main() {
    let num1 = rand::thread_rng().gen_range(1..=10);
    let num2 = rand::thread_rng().gen_range(1..=10);
    println!("What is {} + {}?", num1, num2);
}