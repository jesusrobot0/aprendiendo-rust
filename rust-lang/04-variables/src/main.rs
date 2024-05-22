fn main() {
    // Mut
    // Con mut podemos cambiar solo el valor
    // let mut x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");

    // Las const son como las variables let solo que no se puede usar mut.
    // Y se tiene que especificar el tipo de dato.

    // Shadowing
    // Con el shadowing podemos cambiar el valor y el tipo
    // let x = 5;
    // let x = x + 1;
    // {
    //     let x = x * 2;
    //     println!("The value of x in the inner scope is: {x}");
    // }
    // println!("The value of x is: {x}");

    // Esto no funciona
    // let mut spaces = "   ";
    // spaces = spaces.len();

    let spaces = "   ";
    let spaces = spaces.len();

    print!("{spaces}")
}
