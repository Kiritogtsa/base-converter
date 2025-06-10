use super::{decimal::Decimal, hexadecimal::Hexadecimal};
use regex::Regex;
use std::fmt::Display;

// lsb -> msb
// armazena na ordem do bit menos significativo para o mais significativo
pub struct Binario {
    pub value: Vec<u128>,
}
fn verify_to_string_from_one_or_zero(value: String) -> bool {
    let myregex = Regex::new(r"^[01]+$").unwrap();
    myregex.is_match(value.as_str())
}

impl TryFrom<String> for Binario {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut binary: Vec<u128> = Vec::new();
        if verify_to_string_from_one_or_zero(value.clone()) {
            if value.len() > 128 {
                return Err("o valor digitado ultrapassou o valor maximo");
            }
            for c in value.chars().rev() {
                binary.push(c.to_digit(10).unwrap() as u128);
            }
            Ok(Binario { value: binary })
        } else {
            Err("nao foi possivel conveter a string para binary")
        }
    }
}
impl From<Hexadecimal> for Binario {
    fn from(value: Hexadecimal) -> Self {
        let numerodecimal: Decimal = value.into();
        Self::from(numerodecimal)
    }
}

impl From<u128> for Binario {
    fn from(value: u128) -> Self {
        let mut sum = value;
        let mut binary: Vec<u128> = Vec::new();
        while sum > 0 {
            let res: u128 = sum % 2;
            if res != 0 {
                binary.push(1);
            } else {
                binary.push(0);
            }
            sum /= 2
        }
        Binario { value: binary }
    }
}
impl From<Decimal> for Binario {
    fn from(value: Decimal) -> Self {
        Self::from(value.value)
    }
}
// mais conveções se precisar

impl Display for Binario {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let binary_str = self
            .value
            .iter()
            .rev()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        write!(f, "numero binario: {}", binary_str)
    }
}
