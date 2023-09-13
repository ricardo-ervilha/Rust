use std::process;
use std::io;

struct Node {
    able_actions: Vec<i32>, // Ações disponíveis
    action: i32,            // Última ação tentada
}

struct Pilha {
    pilha : Vec<Node> //Armazena os nós, ou seja, pontos onde eu posso ir para um lugar
}

impl Pilha{

    //Construtor
    fn new() -> Self {
        Pilha { pilha : Vec::new() }
    }

    //Adiciona elementos na pilha
    fn add(&mut self, node : Node){
        self.pilha.push(node);
    }

    //Verifica se a pilha está vazia
    fn vazia(&self) -> bool {
        self.pilha.is_empty()
    }

    //Remove um elemento do topo da pilha e retorna o valor
    fn remove(&mut self){
        if self.vazia() {
            println!("[ERRO] Remoção de elemento de uma pilha vazia!");
            process::exit(1);
        } else{
            self.pilha.pop();
        }
    }

    fn top(&mut self) -> Option<&mut Node> {
        self.pilha.last_mut()
    }
}

fn inicializa_matriz(matriz : &mut Vec<Vec<i32>>, n : usize){
    // Preencho a primeira coluna
    for i in 0..n{
        matriz[i][0] = (i+1) as i32;
    }
}

fn imprime_matriz(matriz : &Vec<Vec<i32>>, n : usize){
    for i in 0..n{
        for j in 0..n{
            print!("{:3}", matriz[i][j]);
        }
        println!("");
    }
}

fn preenche_regras(matriz: &Vec<Vec<i32>>, n : usize, i : usize, j : usize) -> Vec<i32> {

    //Para cada regra 
    let mut rules : Vec<i32> = Vec::new();

    //Para cada regra possível 
    for r in 1..n+1 {
        let mut flag = true;

        //Testar para coluna
        for x in 0..i{
            if matriz[x][j] == (r as i32) {
                flag = false;
                break;
            }
        }

        if flag{
            
            //Testar para linha
            for y in 0..j{
                if matriz[i][y] == (r as i32){
                    flag = false;
                    break;
                }
            }
        }

        //Adiciono na lista de regras que podem ser utilizadas 
        if flag {
            rules.push(r as i32);
        }
    } 

    return rules;
}

//Função para executar a busca em profundidade
fn dfs(matriz: &mut Vec<Vec<i32>>, n : usize){
    //Ponto de partida
    let mut i : usize = 0;
    let mut j : usize = 1;

    //Pilha para armazenar as decisões
    let mut pilha = Pilha::new();
    
    let mut backtracking = false;
    loop{
        if i < n && j < n {

            if !backtracking {
                let mut regras_disponives = preenche_regras(&matriz, n, i, j);
            
                if !regras_disponives.is_empty() {
                    let action = regras_disponives.remove(0);
                    let node = Node{
                        able_actions : regras_disponives,
                        action : action
                    };
                    pilha.add(node);
                    matriz[i][j] = action;
                    if i < n-1{
                        i += 1;
                    }else{
                        i = 0; j += 1;
                    }
                }else{
                    backtracking = true; //Estou em estado de backtracking 
                    if i > 0{
                        i -= 1;
                    }else{
                        i = n - 1; j -= 1;
                    }
                }
            } else {
                let top : &mut Node = pilha.top().unwrap(); //Pego o topo da pilha
    
                if top.able_actions.is_empty() {
                    pilha.remove(); //Removo o topo da pilha
                    if i > 0{
                        i -= 1;
                    }else{
                        i = n - 1; j -= 1;
                    }
                }else{
                    //Para não calcular regras disponives de novo
                    let action = top.able_actions.remove(0);
                    top.action = action;
                    matriz[i][j] = action;
                    if i < n-1{
                        i += 1;
                    }else{
                        i = 0; j += 1;
                    }
                    backtracking = false; //Sai do estado de backtracking
                }
            }
    
        }else{
            println!("i: {}, j: {}", i, j);
            println!("Solução encontrada");
            imprime_matriz(&matriz, n);
            process::exit(1);
        }
        
    }
    
}

fn main(){
    println!("*-*-* Quadrado Latino *-*-*");
    println!("Digite o valor de n: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Falha ao ler o valor do n!");

    let n : i32 = input.trim().parse().unwrap();

    let mut matriz : Vec<Vec<i32>> = vec![vec![0; n as usize]; n as usize];

    inicializa_matriz(&mut matriz, n as usize);
    
    dfs(&mut matriz, n as usize)
}