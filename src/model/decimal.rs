use std::fmt::Display;

use super::{binario, hexadecimal::Hexadecimal};

pub struct Decimal {
    pub value: u128,
}

impl TryFrom<binario::Binario> for Decimal {
    type Error = &'static str;
    fn try_from(value: binario::Binario) -> Result<Self, Self::Error> {
        if value.value.clone().len() > 128 {
            return Err("o binario e muito grande, maior de 128bits");
        }
        let mut sum: u128 = 0;
        for (i, b) in value.value.iter().enumerate() {
            if *b == 1 {
                let result: u128 = 2_u128.pow(i as u32);
                sum += result;
            }
        }
        Ok(Decimal { value: sum })
    }
}

impl TryFrom<String> for Decimal {
    type Error = std::num::ParseIntError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.parse::<u128>() {
            Ok(number) => Ok(Decimal { value: number }),
            Err(e) => Err(e),
        }
    }
}
impl From<Hexadecimal> for Decimal {
    fn from(value: Hexadecimal) -> Self {
        let mut sum: u128 = 0;
        let power = value.value.len();
        for (i, hexanumber) in value.value.iter().enumerate() {
            sum += hexanumber * 16_u128.pow((power - 1 - i) as u32);
        }
        Decimal { value: sum }
    }
}

impl Display for Decimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mystring: String = self.value.to_string();
        write!(f, "number: {}", mystring)
    }
}
