fn main() {
    /*Tipos primitivos/simples*/  
    // let a : i32 = 2; //Tipo inteiro com 32 bits
    // let b : u32 = 4; //Tipo unsigned com 32 bits
    // let c : f32 = 4.654; //Tipo float com 32 bits
    // let d : bool = true; //Tipo booleano
    // let e : char = 'c'; //Tipo char

    let tuple = (1, true, 'c', 3.14);
    //Elementos da tupla são indexados com `.`  
    println!("{}", tuple.0);

    //Usando a key-word mut, podemos alterar os elementos de uma tupla   
    let mut tuple2 = (2, false, ';', 2.71);
    tuple2.0 = 10;
    println!("{}", tuple2.0);

    //Sem usar o mut não podemos mexer nos valores da lista  
    let mut array1 = [1,2,3,4,5];
    //Diferente das tuplas, o índice do array é acessado com colchetes
    println!("{}", array1[0]);
    array1[0] = -15;
    println!("{}", array1[0]);

    //Alguns cuidados que devemos ter
    let x: u8 = 4;
    let y = x;
    println!("{}, {}", x, y);  
    //Nesse caso o y receberá mesmo valor e mesmo tipo de x.
}
