use std::io;

fn main() {
    // Tipos Escalares -------------------------
    // let number = 1;
    // let float = 3.1416;
    // let char = '🦀';

    // Operaciones
    // suma
    // let sum = number + number;
    // resta
    // let difference = number - 4;
    // multiplicación
    // let product = number * 2;
    // división
    // let quotient = float / 32.2;
    // let truncated = -5 / number;
    // resto
    // let remainder = 43 % 5;

    // println!("{char} :  {sum} {difference} {product} {quotient} {truncated} {remainder}")

    // Tipos compuestos ---------------------

    // tuplas
    // let (x, y, z) = (1, 2, 3);
    // print!("{x} {y} {z}")

    // arrays

    // let array: [i32; 5] = [1, 2, 3, 4, 5];
    // println!("{}", array[3])
    // println!("{}", array[7]) // error en build time

    // panic --------------------------------
    // error en run time
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
