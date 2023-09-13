use std::process;
use std::collections::HashSet;
// use std::io;

struct Node {
    estado : Vec<i32>,
    pai : &Node,
    acao: i32
}

struct Fronteira{
    fronteira : Vec<Node>
}

//Fronteira funciona como uma pilha
impl Fronteira{
    //Construtor, cria e retorna uma fronteira vazia.  
    fn new() -> Self{
        Fronteira { fronteira : Vec::new() }
    }

    //Adiciona elemento na fronteira.  
    fn add(&mut self, node : Node) {
        self.fronteira.push(node);
    } 

    //Verifica se um certo estado qualquer está na fronteira
    fn contem(&self, state : Vec<i32>) -> bool{

        //Para cada estado da fronteira  
        for val in &self.fronteira{

            //Fico comparando cada elemento com o do que foi passado
            for i in *(val).len(){
                if *(val + i) != state[i]{
                    break;
                }
            }

            if(i == *(val).len()){
                return true;
            }
        }

        return false;
    }

    //Verifica se a fronteira está vazia
    fn vazia(&self) -> bool{
        self.fronteira.is_empty()
    }

    //Remove elemento da fronteira e retorna o valor
    fn remove(&mut self) -> Option<i32>{
        if self.fronteira.is_empty() {
            println!("[ERRO] Removendo elemento com fronteira vazia!");
            process::exit(1);
        }else{
            self.fronteira.pop()
        }
    }
}

fn vizinhos(estado : Vec<i32>, n : i32){
    let acoes : Vec<i32> = Vec<i32>::new();

    for i in 1..n+1{
        let flag = true;
        for j in 0..(estado.len()/n as i32){
            if(i+1 == estado[j*n + (i-1)]){
                flag = false;
                break;
            }
        }
        for j in 1..(estado.len() % n as i32){
            if(estado[0] == estado[j])
            {
                flag = false;
                break;
            }
        }
        if flag{
            acoes.push(i);
        }
    } 

    return acoes;
}

//Busca em profundidade
fn dfs(let n : i32) -> bool{
    let mut estados_explorados : i32 = 0;

    let estadoInicial = Node {
        estado = Vec<i32>::new(), //estado atual começa com vazio
        pai = None, //Nenhum pai
        acao = None //Nenhuma ação levou aquele cara ali
    }

    let mut fronteira = Fronteira::new();

    fronteira.add(estadoInicial);

    let mut estados_explorados = HashSet::new();

    //Explorar até achar uma solução
    while true{

        if fronteira.vazia(){
            println!("Nenhuma solução encontrada!");
            process::exit(1);
        }

        //Escolhe o nó no topo da fronteira
        let no = fronteira.remove();

        estados_explorados += 1;

        //Se cair aqui dentro, é porque achou uma solução
        if no.estado.len() == n*n - n {
            //Agora devemos caminhar para trás e achar qual foi a solução
            return true;
            // let regras_usadas : Vec<i32> = Vec::new();

            // while no.parent is not None{
            //     regras_usadas.push(no.regras);
            //     no = no.pai;
            // }

            // regras_usadas.reverse();

            // return regras_usadas;
        }

        //Marca aquele estado como explorado
        estados_explorados.push(no.estado);

        //Adiciona vizinhos na fronteira
        for acao em vizinhos(no.estado, n){
            if !fronteira.contem(no.estado.push(acao)) && !estados_explorados.contains(no.estado.push(acao)){
                filho = Node{
                    estado : no.estado.push(acao),
                    pai : no
                    acao: acao
                };

                fronteira.add(filho);
            }
        }
    }
}

fn main() {

    println!("*-*-* Quadrado Latino *-*-*");
    println!("Digite o valor de n: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Falha ao ler o valor de n!");

    //Conversão para inteiro da string lida
    let n : usize = input.trim().parse().unwrap();
    
    println!("{}", dfs(n));
}
