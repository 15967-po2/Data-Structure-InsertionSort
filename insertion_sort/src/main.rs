/*
Insertion Sort Algorithm
main.rs
Nuno Cruz
2021.03.16

utilização de vetores como tabelas com utilização dinâmica da memória

- tabelas de dimensão definida em tempo de compilação (arrays)
- tabelas de dimensão definida em tempo de execução (Vec)
 */
use rand::Rng;
use std::time::Instant;
//use rand::prelude::*;

/*
    Função Main
*/
fn main() {
    const N: usize = 1000;
    const NTIME: usize = 1000;
    const NSIZE: usize = 1000; // Tamanho máximo do vector

    let mut t_vec: Vec<f64> = vec![0.0; NTIME]; // Vetor de tempos inicializado com o tamanho da constante NTIME
    let mut n: usize = 100;
    let mut i = 0;


    while n < NSIZE {
        let mut k = 0;

        // Executa 10 vezes o programa e dá print dos tempos de execução de todas as vezes e guarda no vetor
        while k < NTIME {
            let now = Instant::now(); // Cria um timer

             
            let mut a = random_table(N);    // Chama método random_table()

            let t_inicial = now.elapsed().as_nanos();
            insertion_sort(&mut a);
            let t_final = now.elapsed().as_nanos();
            let t_dif = t_final - t_inicial;

            t_vec[k] = t_final as f64;  // Guarda os vários tempos de execução num Vector<f64>

            k += 1;
        }

        println!(" ");
        println!(" {}", n);
        println!("Média de execução de ordenação: {:?}", media(&t_vec));
        println!("Desvio Padrão de execução de ordenação: {:?}", desvio_p(&t_vec));
        let r: f64 = desvio_p(&t_vec) / media(&t_vec) * 100.0;
        println!("Erro relativo de ordenação: {} %", r);

        n += 100;
    }

    let mut total: f64 = 0.0;
    while i < t_vec.len() {
        total = total + t_vec[i];
        i+=1;
    }

    let m_final: f64 = total / 1000.0;
    println!("Media final: {} nanossegundos", m_final);

}


/*
    Função que gera um vector de números aleatórios
    @n <i32>
    @return a_vec Vec<i32>
*/
fn random_table(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut a_vec: Vec<i32> = vec![0; n];

    let mut i = 0;
    while i < n {
        let a: i32 = rng.gen_range(0..100);
        a_vec[i] = a;
        i += 1;
    }
    a_vec
}


/*
    Função algoritmo insertion sort
    @a Vec<i32> (valor passado por referência)
*/
fn insertion_sort(a: &mut Vec<i32>) {
    //let mut i:usize = 1;
    let mut j: usize = 1;
    //let mut key = 0;

    while j < a.len() {
        let mut i: usize = j - 1;
        let key = a[j];

        while i > 0 && a[i] > key {
            a[i + 1] = a[i];
            i -= 1;
        }

        a[i + 1] = key;
        j += 1;
    }
}


/*
    Função que calcula desvio padrão
    @x Vec<f64> (Valor passado por referência)
    @return m <f64>
*/
fn media(x: &Vec<f64>) -> f64 {
    let mut m: f64 = 0.0;

    let mut i: usize = 0;
    while i < x.len() {
        m += x[i];
        i += 1;
    }

    m = m / x.len() as f64;
    m
}


/*
    Função que calcula o desvio padrão
    @x Vec<f64> Valor passado por referência
    @return s <f64>
*/
fn desvio_p(x: &Vec<f64>) -> f64 {
    let mut s: f64 = 0.0;
    let mut i: usize = 0;
    let m = media(&x);

    while i < x.len() {
        s += (x[i] - m) * (x[i] - m);
        i += 1;
    }

    s = s / x.len() as f64;
    s = s.sqrt();
    s
}
