use std::collections::HashMap;
use std::io::{self, Write};

// Dados de treino: (texto, rótulo). 1 = gasto, 0 = anotação.
// DICA: adicione mais exemplos seus aqui e rode de novo!
const DADOS: &[(&str, i32)] = &[
    ("paguei 50 reais no mercado", 1),
    ("comprei pão por 8 reais", 1),
    ("gastei 120 no uber", 1),
    ("almoço 35 reais", 1),
    ("paguei a conta de luz 210", 1),
    ("comprei um livro por 60", 1),
    ("gasolina 150 reais", 1),
    ("ligar para o dentista amanhã", 0),
    ("ideia: estudar rust no fim de semana", 0),
    ("reunião com o time na sexta", 0),
    ("lembrar de ler sobre machine learning", 0),
    ("anotar a senha do wifi com a Ana", 0),
    ("ver aquele filme que o João indicou", 0),
    ("estudar equação linear hoje", 0),
];

/// Quebra o texto em palavras minúsculas. Números viram "<num>".
fn palavras(texto: &str) -> Vec<String> {
    texto
        .to_lowercase()
        .split_whitespace()
        .map(|p| p.trim_matches(|c: char| !c.is_alphanumeric()).to_string())
        .filter(|p| !p.is_empty())
        .map(|p| if p.parse::<f64>().is_ok() { "<num>".to_string() } else { p })
        .collect()
}

/// A equação linear:  score = b + w1*x1 + w2*x2 + ...
/// onde xi = 1 se a palavra i aparece no texto, senão 0.
struct Modelo {
    vocab: HashMap<String, usize>, // palavra -> posição
    pesos: Vec<f64>,               // w
    vies: f64,                     // b
}

impl Modelo {
    fn treinar(dados: &[(&str, i32)], epocas: usize) -> Modelo {
        // 1) monta o vocabulário
        let mut vocab = HashMap::new();
        for (texto, _) in dados {
            for p in palavras(texto) {
                let n = vocab.len();
                vocab.entry(p).or_insert(n);
            }
        }
        let mut m = Modelo { pesos: vec![0.0; vocab.len()], vocab, vies: 0.0 };

        // 2) perceptron: se errou, empurra os pesos na direção certa
        for _ in 0..epocas {
            for (texto, alvo) in dados {
                let erro = (*alvo - m.prever(texto)) as f64; // -1, 0 ou +1
                if erro != 0.0 {
                    for p in palavras(texto) {
                        if let Some(&i) = m.vocab.get(&p) {
                            m.pesos[i] += erro;
                        }
                    }
                    m.vies += erro;
                }
            }
        }
        m
    }

    fn score(&self, texto: &str) -> f64 {
        let mut s = self.vies;
        for p in palavras(texto) {
            if let Some(&i) = self.vocab.get(&p) {
                s += self.pesos[i];
            }
        }
        s
    }

    fn prever(&self, texto: &str) -> i32 {
        if self.score(texto) > 0.0 { 1 } else { 0 }
    }
}

fn main() {
    let modelo = Modelo::treinar(DADOS, 20);

    println!("== Pesos aprendidos (positivo = puxa para 'gasto') ==");
    let mut ordenado: Vec<_> = modelo.vocab.iter().collect();
    ordenado.sort_by(|a, b| {
        modelo.pesos[*b.1].partial_cmp(&modelo.pesos[*a.1]).unwrap()
    });
    for (palavra, &i) in ordenado {
        if modelo.pesos[i] != 0.0 {
            println!("{:>12}  {:+.1}", palavra, modelo.pesos[i]);
        }
    }
    println!("{:>12}  {:+.1}\n", "(viés b)", modelo.vies);

    println!("Digite um texto (ou 'sair'):");
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut linha = String::new();
        if io::stdin().read_line(&mut linha).unwrap() == 0 { break; }
        let linha = linha.trim();
        if linha == "sair" { break; }
        let s = modelo.score(linha);
        let rotulo = if s > 0.0 { "GASTO" } else { "ANOTAÇÃO" };
        println!("   score = {:+.1}  =>  {}", s, rotulo);
    }
}
