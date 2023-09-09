use std::io;

fn main() {
    //Testando input data e op. aritméticas com type casting  
    let x = 255.0; 
    let y = 10.0; 

    let z = x / y;
    println!("The value of z is: {}", z);

    let a = 64_i8;
    let b = 10_i8;

    let c = a + b;
    println!("The value of c is: {}", c);

    //Utilizamos `as` para fazer cast de variáveis
    let as1 = 255.2543 as i32;
    let as2 = 100.4235 as i32;

    let as_result = as1 + as2;
    println!("The value of asResult is: {}", as_result);

    println!("Digite alguma coisa:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");

    let int_input : i64 = input.trim().parse().unwrap();

    println!("{}", int_input + 5);

}
