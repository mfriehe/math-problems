fn main() {
    let mut rng = rand::thread_rng();
    let x: i32 = rng.gen_range(0..10);
    let y: i32 = rng.gen_range(0..10);
    println!("What is {} + {}?", x, y);
}
