use std::process;
use std::io;

struct Node {
    able_actions: Vec<i32>, // Ações disponíveis
    already_attempt: i32,   // Última ação tentada
}

struct Pilha {
    pilha: Vec<Node>,
}

// Pilha de nós
impl Pilha {
    // Construtor, cria e retorna uma fronteira vazia.
    fn new() -> Self {
        Pilha { pilha: Vec::new() }
    }

    // Adiciona elemento na fronteira.
    fn add(&mut self, node: &'static mut Node) {
        self.pilha.push(node);
    }

    // Verifica se a fronteira está vazia
    fn vazia(&self) -> bool {
        self.pilha.is_empty()
    }

    fn top(&mut self) -> Option<&mut Node> {
        self.pilha.last_mut().map(|x| *x)
    }

    // Remove elemento da fronteira e retorna o valor
    fn remove(&mut self) -> Option<&'static mut Node> {
        if self.pilha.is_empty() {
            println!("[ERRO] Removendo elemento com fronteira vazia!");
            process::exit(1);
        } else {
            self.pilha.pop()
        }
    }
}

fn inicializa_matriz(matriz: &mut Vec<Vec<i32>>, n: usize) {
    for i in 0..n {
        matriz[i][0] = (i + 1) as i32;
    }
}

fn imprime_matriz(matriz: &Vec<Vec<i32>>, n: usize) {
    for i in 0..n {
        for j in 0..n {
            print!("{:3}", matriz[i][j]);
        }
        println!();
    }
}

fn preenche_regras(matriz: &Vec<Vec<i32>>, n: i32, i: usize, j: usize) -> Vec<i32> {
    // Para cada regra
    let mut rules: Vec<i32> = Vec::new();
    for r in 1..n + 1 {
        let mut flag = true;

        // Testar para coluna
        for x in 0..i {
            if matriz[x][j] == r {
                flag = false;
                break;
            }
        }

        if flag {
            // Testar para linha
            for y in 0..j {
                if matriz[i][y] == r {
                    flag = false;
                    break;
                }
            }
        }

        if flag {
            rules.push(r);
        }
    }

    return rules;
}

fn dfs(matriz: &mut Vec<Vec<i32>>, n: i32) {
    let mut i: usize = 0;
    let mut j: usize = 1;
    let mut pilha = Pilha::new();

    let mut cond = true;
    loop {
        if i < n as usize && j < n as usize {
            let mut regras_disponiveis: Vec<i32>;

            if cond {
                regras_disponiveis = preenche_regras(&matriz, n, i, j);

                // Estado de impasse
                if regras_disponiveis.is_empty() {
                    // Troco cond pra falso para não preencher regras novamente
                    cond = false;
                } else {
                    // Salvo esse estado
                    let regra = regras_disponiveis.pop();
                    let no = Node {
                        able_actions: regras_disponiveis.clone(), // Clone para evitar problemas de mutabilidade
                        already_attempt: regra.unwrap(),
                    };
                    pilha.add(Box::leak(Box::new(no))); // Box::leak para armazenar uma referência estática
                    matriz[i][j] = regra.unwrap();

                    // Atualizo para ir aprofundando na árvore
                    if i < n as usize {
                        i += 1;
                    } else {
                        i = 0;
                        j += 1;
                    }
                }
            } else {
                if !pilha.vazia() {
                    // Clona o nó superior da pilha
                    let mut top = pilha.top().unwrap().clone();

                    // Atualiza o pai para tentar a próxima regra que ele permite
                    let regra = top.able_actions.pop();
                    top.already_attempt = regra.unwrap();
                    matriz[i][j] = regra.unwrap();
                    cond = true;

                    if i < n as usize {
                        i += 1;
                    } else {
                        i = 0;
                        j += 1;
                    }
                } else {
                    matriz[i][j] = 0;
                    if !pilha.vazia() {
                        pilha.remove();
                        if i > 0 {
                            i -= 1;
                        } else {
                            i = n as usize - 1;
                            j -= 1;
                        }
                    } else {
                        println!("Nenhuma solução encontrada.");
                        process::exit(1);
                    }
                }
            }
        } else {
            break;
        }
    }
}

fn main() {
    println!("*-*-* Quadrado Latino *-*-*");
    println!("Digite o valor de n: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Falha ao ler o valor de n!");

    // Conversão para inteiro da string lida
    let n: i32 = input.trim().parse().unwrap();

    // Criação da matriz dinâmica nxn preenchida com zeros
    let mut matriz: Vec<Vec<i32>> = vec![vec![0; n as usize]; n as usize];

    inicializa_matriz(&mut matriz, n as usize);

    dfs(&mut matriz, n);
    imprime_matriz(&matriz, n as usize);
}
