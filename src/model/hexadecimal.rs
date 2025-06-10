// faço o do octa ou não?
use super::{
    binario::{self, Binario},
    decimal::{self, Decimal},
};
use regex::Regex;
use std::fmt::Display;

// lsb -> msb
// armazena na ordem do bit menos significativo para o mais significativo
pub struct Hexadecimal {
    pub value: Vec<u128>,
}

impl From<decimal::Decimal> for Hexadecimal {
    fn from(value: decimal::Decimal) -> Self {
        let mut sum: u128 = value.value;
        let mut hexa: Vec<u128> = Vec::new();
        if sum == 0 {
            return Hexadecimal { value: vec![0] };
        }
        while sum > 0 {
            hexa.push(sum % 16);
            sum /= 16;
        }
        Hexadecimal { value: hexa }
    }
}
fn verify_string_for_hexadecimal(value: String) -> bool {
    let myregex = Regex::new(r"^[0-9a-fA-F]+$").unwrap();
    myregex.is_match(value.as_str())
}
impl TryFrom<Binario> for Hexadecimal {
    type Error = &'static str;
    fn try_from(value: binario::Binario) -> Result<Self, Self::Error> {
        let result: Result<Decimal, Self::Error> = value.try_into();
        match result {
            Ok(decimal) => Ok(Self::from(decimal)),
            Err(_) => Err("erro ao pegar o numero decimal"),
        }
    }
}
impl TryFrom<String> for Hexadecimal {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        // limite de 32 characteres
        // implementar depois
        let mut hexadecimal: Vec<u128> = Vec::new();
        let letras = &[
            'A', //0
            'B', //1
            'C', //2
            'D', //3
            'E', //4
            'F', //5
        ];
        if value.len() > 32 {
            return Err("o valor ultrapassa o limite de 32bits para o hexadecil");
        }
        if verify_string_for_hexadecimal(value.clone()) {
            let chars = value.to_uppercase().chars().rev().collect::<Vec<char>>();
            let mut i = 0;
            while i < chars.len() {
                let c = chars[i];
                let mut numero: u128 = 0;
                if c >= '0' && c <= '9' {
                    numero = (c as u8 - b'0') as u128;
                } else if c >= 'A' && c <= 'F' {
                    let mut j = 0;
                    while j < letras.len() {
                        if letras[j] == c {
                            numero = j as u128 + 10;
                            break;
                        }
                        j += 1;
                    }
                } else {
                    return Err("Caractere inválido no hexadecimal");
                }

                hexadecimal.push(numero);
                i += 1;
            }
            Ok(Hexadecimal { value: hexadecimal })
        } else {
            Err("String inválida para hexadecimal")
        }
    }
}
impl Display for Hexadecimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let letras = &[
            'A', //0
            'B', //1
            'C', //2
            'D', //3
            'E', //4
            'F', //5
        ];
        let mut my_string = String::new();
        for value in self.value.iter().rev() {
            if *value >= 10 && *value < 16 {
                my_string += letras[(*value as usize) - 10].to_string().as_str();
            } else {
                my_string += value.to_string().as_str();
            }
        }
        write!(f, "o numero em hexadecimal: {}", my_string)
    }
}
