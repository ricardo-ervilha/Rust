fn main() {
    //Na stack só podemos adicionar coisas de tamanho fixo, ou seja, vetores ou outras estruturas de dados não ficam na stack
    
    //A stack é mais rápida do que a heap
    
    //Na heap irão ficar estruturas dinâmicas que podem variar o tamanho em bytes
    
    //Se passarmos variáveis sem mut como parâmetros as mesmas são passadas por cópia (?)
    
    //a = "Teste" (a pode ser armazenado na stack, pois a string tem tamanho fixo)
    
    //a = String::from("Teste") (a não pode ser na stack, pois a string tem tamanho variável) -> Nesse caso, teremos um ponteiro na stack que aponta para a posição onde o valor "Teste" estará na heap.

    /*Ownership rules*/
    // 1. Cada valor no rust tem uma variável chamada de owner. (Uma variável <=> Um owner)
    // 2. Só se pode ter um owner de cada vez. (Uma variável não pode ter dois owners ao mesmo tempo)
    // 3. Quando o owner sai do escopo, o valor será descartado.

    //Exemplo

    {//s não é valido aqui, ainda não está declarado
        let s : &str = "hello"; // s é válido desse ponto em diante
        //faço coisas com s

        //Se s fosse declarada como String, rust alocaria memória para a variável, e, após sair desse escopo, automaticamente desalocaria a memória.
    }//esse escopo agora está terminado, e s não tem mais validade


    let x : i32 = 5;
    let y : i32 = x; //Valor aqui será passado por cópia
    
    let s1 : String = String::from("hello");
    let s2 : String = s1; //move (now shalow copy)
    //Se quisermos clonar s1: let s2 : String = s1.clone()
}
