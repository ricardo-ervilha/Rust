// Código abaixo não está tão bem escrito

fn main() {
    let s1: String = String::from("hello");
    let (s2 : String, len: usize) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize){
    let length: usize = s.len();  //Retorna o tamanho da string
    (s,length)
}

// Melhorias que podemos fazer
// Passar por referência é chamado de empréstimo

fn main() {
    let s1: String = String::from("hello");
    let len: usize = calculate_length(&s1); //Podemos passar um ponteiro para ser alterado na função abaixo
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    //Referências por padrão são imutáveis  
    let length: usize = s.len();  //Retorna o tamanho da string
    length
}

/****************************************************************************************/

fn main(){
    let s1: String = String::from("hello");
    change(&s1);
}

fn change(some_string : &String){
    //A linha abaixo acusará erro, pois não podemos alterar o que está na referência
    //Referências são imutáveis
    //Para poder modificar o conteúdo, teríamos que declarar s1 como mutable, e passar uma referência mutavel como (&mut s1)    
    some_string.push_str(", world!");
}

/****************************************************************************************/


fn main(){
    //Teremos um erro na linha 52, visto que não podemos emprestar uma variável mais de uma vez no mesmo escopo.
    //O benefício dessa restrição é que o rust pode data races em tempo de compilação 
    //Uma data race ocorre se tivermos dois ponteiros apontando para o mesmo pedaço de dados e um dos ponteiros é utilizado para escrever nos dados, não havendo mecanismo para sincronizar o acesso as informações. Nessa situação, um dos ponteiros pode tentar ler os dados no meio da escrita dos dados do outro ponteiro, e assim, obteremos dados corrompidos.
    //Para corrigir isso devemos voltar s para imutable.  

    let mut s : String = String::from("hello");

    let r1 : String = &mut s;
    let r2 : String = &mut s;

    println!("{}, {}", r1, r2);
}