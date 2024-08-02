use std::fmt::Display;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Radix {
    Uns = 0,
    Bin = 1,
    Oct = 2,
    Dec = 3,
    Hex = 4,
}

impl Radix {
    pub fn radix(&self) -> u32 {
        match self {
            Self::Uns => 10,
            Self::Bin => 2,
            Self::Oct => 8,
            Self::Hex => 16,
            Self::Dec => 10,
        }
    }
}

impl Radix {
    pub fn digits(&self) -> &'static str {
        &"0123456789abcdefABCDEF"[0..match self {
            Self::Uns | Self::Dec => 10,
            Self::Bin => 2,
            Self::Oct => 8,
            Self::Hex => 22,
        }]
    }
}

impl Display for Radix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Bin => "BIN",
            Self::Hex => "HEX",
            Self::Oct => "OCT",
            Self::Uns => "UNS",
            Self::Dec => "DEC",
        })
    }
}
