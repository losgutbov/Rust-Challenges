use std::io;

fn main() {
    //OPERAÇÕES
    let sum = 5 + 10;
    println!("{sum}");

    let difference = 95.5 - 4.3;
    println!("{difference}");

    let product = 4 * 30;
    println!("{product}");

    let quatient = 56.7 / 32.2;
    let truncated = -5 / 3;
    println!("{quatient} {truncated}");

    let remainder = 43 % 5;
    println!("{remainder}");

    //TUPLE
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("O valor de y é: {y}");

    println!("O valor da primeira posição de tup é: {}", tup.0);

    //ARRAY
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
