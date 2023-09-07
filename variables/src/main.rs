fn main() {
    /*Uso do `mut` para deixar x como variável mutável*/
    // let mut x = 4;
    // println!("x is: {}", x);
    // x = 5;
    // println!("x is: {}", x);

    /*Podemos redefinir o valor de uma variável dentro do mesmo escopo*/
    let x = 4;
    println!("x is: {}", x);
    let x = 5;
    println!("x is: {}", x); 

    {
        //Criamos um novo escopo dentro da função  
        let x = x + 2;
        println!("x is: {}", x);
    }

    let x = x + 1;
    println!("x is: {}", x);

    //Redefinindo a variável podemos driblar a questão estática de variáveis do rust
    let x = "hello";
    println!("x is: {}", x);

    const SECONDS_IN_MINUTES : i32 = 60;
    // const SECONDS_IN_MINUTES : u32 = 100; //Não podemos redefinir uma constante, essa linha ira nos retornar um erro
    println!("{}", SECONDS_IN_MINUTES);
}
