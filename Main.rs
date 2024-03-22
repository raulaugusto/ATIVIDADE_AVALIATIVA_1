use std::fs::File;
use std::io::{BufReader, stdin};
use std::path::Path;
use std::io::{self, Write};

fn main() {

    let mut soma = 0;
    let mut contador = 0;
    let mut first_line = 1;
    let mut last_line = 11;
    let mut o: String = String::new();
    let path = Path::new("./data/matriz.json");
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);
    let matriz: Vec<Vec<i32>> = serde_json::from_reader(reader).unwrap();

    for row in &matriz {
        // Print the elements of the row separated by a space
        println!("{}", row.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(", "));
    }

    println!("Digite a operação desejada: ");

    stdin().read_line(&mut o).unwrap();
    let o = o.trim();
    
    println!("Números selecionados: ");
    for i in (7..12).rev() {
    let mut tracker = first_line; // Initialize tracker with first_line
    while tracker < last_line {
        println!("[Linha: {} - Coluna: {}] => {}",tracker, i, matriz[tracker][i]);
        soma += matriz[tracker][i];
        tracker += 1;
        contador += 1;
    }
    first_line += 1;
    last_line -= 1;
}

    println!();
    if o == "M" {
        println!("Soma S ou Média M => Média: {}", soma / contador);
    } else if o == "S"{
        println!("Soma S ou Média M => Soma: {}", soma);
    }else{
        print!("Opção Inválida (M ou S)")
    }
    println!();
    println!("Representação da Matriz: ");
    println!();
    print_array();
}
fn print_dots_and_xs(chars: usize, max_chars: usize) {
    let mut remaining_chars = chars;
    while remaining_chars > 0 {
        print!(".  ");
        remaining_chars -= 1;
    }
    let num_xs = max_chars - chars;
    for _ in 0..num_xs {
        print!("X  ");
    }
    println!();
}

fn print_array() {
    let mut chars = 12;
    for _i in 0..6 {
        print_dots_and_xs(chars, 12);
        chars -= 1;
    }

    chars = 7;
    for _i in 0..6 {
        print_dots_and_xs(chars, 12);
        chars += 1;
    }
    println!("Press Enter to exit...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
}

