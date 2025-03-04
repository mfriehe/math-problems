fn main() {
    let mut num1 = 0;
    let mut num2 = 0;
    let mut op = "".to_string();
    println!("Please enter two numbers separated by a space: ");
    scanf("%d %c", &mut num1, &mut op, &mut num2);
    match op.as_str() {
        "+ => println!("The sum is {}", num1 + num2),
        "- => println!("The difference is {}", num1 - num2),
        "*" => println!("The product is {}", num1 * num2),
        "/" => println!("The quotient is {}", num1 / num2),
        _ => println!("Invalid operation"),
    }
}
