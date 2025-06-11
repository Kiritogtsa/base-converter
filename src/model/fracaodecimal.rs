use std::fmt::Display;

use super::{decimal::Decimal, hexadecimal::Hexadecimal};

pub struct FracaoDecimal {
    numerador: u128,
    denominator: u128,
}

impl TryFrom<String> for FracaoDecimal {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        todo!()
    }
}
impl From<Decimal> for FracaoDecimal {
    fn from(value: Decimal) -> Self {
        todo!()
    }
}
impl From<Hexadecimal> for FracaoDecimal {
    fn from(value: Hexadecimal) -> Self {
        todo!()
    }
}
impl Display for FracaoDecimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
