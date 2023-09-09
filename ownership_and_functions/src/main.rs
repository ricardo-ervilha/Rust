fn main() {
    
    let s: String = String::from("Hello");
    takes_ownership(s);
    //Se tentarmos imprimir s depois de chamar a função acima receberemos um erro: s não pode ser emprestado após um movimento. Isso acontece porque quando passamos parâmetros é o mesmo que se atribuíssemos s a outra variável. Passar s move s para outra variável.
    println!("{}", s);
}

fn takes_ownership(some_string: String){
    //Após imprimir some_string a variável s da main é descartada.
    println!("{}", some_string);
}


/****************************************************************************************/

//Outro exemplo

fn main(){
    let s: String = String::from("hello");
    takes_ownership(s);
    println!("{}", s);

    //Nessa parte x irá ser imprimido normalmente, visto que é passado por cópia
    let x: i32 = 5;
    makes_copy(x);
    println!("{}", x);
}

fn takes_ownership(some_string: String){
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32){
    println!("{}", some_integer);
}


/****************************************************************************************/

//Outro exemplo

fn main(){
    //A string criada em gives_ownership será movida para s1 e podemos usá-la depois
    let s1: String = gives_ownership();
    println!("s1 = {}", s1);
}

fn gives_ownership() -> String {
    let some_string: String = String::from("hello");

    //Criamos e retornamos some_string
    some_string
}