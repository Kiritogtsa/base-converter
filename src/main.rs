use std::io::{self, Write};
mod model;
use model::{binario::Binario, decimal::Decimal, hexadecimal::Hexadecimal};
fn main() {
    loop {
        show_menu();
        let choice = read_input("Digite sua escolha: ");

        match choice.trim() {
            "1" => decimal_to_binary_flow(),
            "2" => decimal_to_hexadecimal(),
            "3" => binary_to_decimal_flow(),
            "4" => binary_to_hexadecimal(),
            "5" => hexadecimal_to_decimal(),
            "6" => hexadecimal_to_binario(),
            "q" => {
                println!("Saindo...");
                break;
            }
            _ => println!("Opção inválida, tente novamente."),
        }
    }
}

fn decimal_to_hexadecimal() {
    let input = read_input("digite um numero decimal: ");
    match Decimal::try_from(input) {
        Ok(decimal) => {
            let hexadecimal: Hexadecimal = decimal.into();
            println!("{:}", hexadecimal)
        }
        Err(e) => println!("{:}", e),
    }
}

fn binary_to_hexadecimal() {
    let input = read_input("digite um numero binario: ");
    match Binario::try_from(input) {
        Ok(binario) => {
            let ressult: Result<Hexadecimal, &'static str> = binario.try_into();
            match ressult {
                Ok(hexadecimal) => println!("{:}", hexadecimal),
                Err(e) => println!("{}", e),
            }
        }
        Err(e) => println!("{}", e),
    }
}

fn hexadecimal_to_decimal() {
    let input = read_input("digite um numero hexadecimal: ");
    match Hexadecimal::try_from(input) {
        Ok(hexadecimal) => {
            let decimal: Decimal = hexadecimal.into();
            println!("{:}", decimal);
        }
        Err(e) => println!("{}", e),
    }
}

fn hexadecimal_to_binario() {
    let input = read_input("digite um numero hexadecimal: ");
    match Hexadecimal::try_from(input) {
        Ok(hexadecimal) => {
            let binario: Binario = hexadecimal.into();
            println!("{:}", binario)
        }
        Err(e) => println!("{}", e),
    }
}
fn show_menu() {
    println!("\n=== Menu ===");
    println!("1. Converter decimal para binário");
    println!("2. Converter decimal para hexadecimal");
    println!("3. Converter binário para decimal");
    println!("4. Converter binario para hexadecimal");
    println!("5. Converter hexadecimal para decimal");
    println!("6. Converter hexadecimal para binario");
    println!("q. Sair");
}

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Falha ao ler entrada");
    input.trim().to_string()
}

fn decimal_to_binary_flow() {
    let input = read_input("Digite um número decimal: ");
    match Decimal::try_from(input) {
        Ok(num) => {
            let bin: Binario = Decimal::into(num);
            println!("{:}", bin);
        }
        Err(e) => println!("{:}", e),
    }
}

fn binary_to_decimal_flow() {
    let input = read_input("Digite um número binário: ");
    match Binario::try_from(input) {
        Ok(bin_vec) => {
            let dec: Result<Decimal, &'static str> = bin_vec.try_into();
            match dec {
                Ok(decimal) => println!("{:}", decimal),
                Err(_) => println!("erro na conversão de binario para decimal"),
            }
        }
        Err(e) => println!("{:}", e),
    }
}
