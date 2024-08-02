use std::fmt::{Binary, Display, Octal, UpperHex};

use crate::Radix;

pub struct Mif<'a, T> {
    address_radix: Radix,
    data_radix: Radix,
    data: &'a [T],
}

impl<'a, T: ToMif> Mif<'a, T> {
    const BIN_WIDTH: usize = T::WIDTH;
    const HEX_WIDTH: usize = T::WIDTH / 4;
    const OCT_WIDTH: usize = T::WIDTH / 3 + 1;

    pub fn new(data: &'a [T], address_radix: Radix, data_radix: Radix) -> Self {
        Self {
            data,
            address_radix,
            data_radix,
        }
    }
}

impl<'a, T> Display for Mif<'a, T>
where
    T: ToMif,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let depth = self.data.len();
        let width = T::WIDTH;
        let bin_width = Self::BIN_WIDTH;
        let oct_width = Self::OCT_WIDTH;
        let hex_width = Self::HEX_WIDTH;
        let address_radix = self.address_radix;
        let data_radix = self.data_radix;
        let mut data = self.data.iter().enumerate();

        f.write_fmt(format_args!("DEPTH = {depth}\nWIDTH = {width}\nADDRESS_RADIX = {address_radix}\nDATA_RADIX = {data_radix}\nCONTENT BEGIN\n"))?;

        match (address_radix, data_radix) {
            (Radix::Bin, Radix::Bin) => data.try_for_each(|(index, value)| {
                f.write_fmt(format_args!(
                    "{index:0bin_width$b} : {value:0bin_width$b}\n",
                ))
            })?,
            (Radix::Bin, Radix::Dec) => data.try_for_each(|(index, value)| {
                f.write_fmt(format_args!("{index:0bin_width$b} : {value}\n"))
            })?,
            (Radix::Bin, Radix::Hex) => data.try_for_each(|(index, value)| {
                f.write_fmt(format_args!(
                    "{index:0bin_width$b} : {value:0hex_width$X}\n",
                ))
            })?,
            (Radix::Bin, Radix::Oct) => data.try_for_each(|(index, value)| {
                f.write_fmt(format_args!(
                    "{index:0bin_width$b} : {value:0oct_width$o}\n",
                ))
            })?,
            (Radix::Bin, Radix::Uns) => data.try_for_each(|(index, value)| {
                f.write_fmt(format_args!(
                    "{index:0bin_width$b} : {}\n",
                    value.unsigned()
                ))
            })?,

            (Radix::Hex, Radix::Bin) => data.try_for_each(|(index, value)| {
                f.write_fmt(format_args!(
                    "{index:0hex_width$X} : {value:0bin_width$b}\n",
                ))
            })?,
            (Radix::Hex, Radix::Dec) => data.try_for_each(|(index, value)| {
                f.write_fmt(format_args!("{index:0hex_width$X} : {value}\n",))
            })?,
            (Radix::Hex, Radix::Hex) => data.try_for_each(|(index, value)| {
                f.write_fmt(format_args!(
                    "{index:0hex_width$X} : {value:0hex_width$X}\n",
                ))
            })?,
            (Radix::Hex, Radix::Oct) => data.try_for_each(|(index, value)| {
                f.write_fmt(format_args!(
                    "{index:0hex_width$X} : {value:0oct_width$o}\n",
                ))
            })?,
            (Radix::Hex, Radix::Uns) => data.try_for_each(|(index, value)| {
                f.write_fmt(format_args!(
                    "{index:0hex_width$X} : {}\n",
                    value.unsigned()
                ))
            })?,

            (Radix::Oct, Radix::Bin) => data.try_for_each(|(index, value)| {
                f.write_fmt(format_args!(
                    "{index:0oct_width$o} : {value:0bin_width$b}\n",
                ))
            })?,
            (Radix::Oct, Radix::Dec) => data.try_for_each(|(index, value)| {
                f.write_fmt(format_args!("{index:0oct_width$o} : {value}\n"))
            })?,
            (Radix::Oct, Radix::Hex) => data.try_for_each(|(index, value)| {
                f.write_fmt(format_args!(
                    "{index:0oct_width$o} : {value:0hex_width$X}\n",
                ))
            })?,
            (Radix::Oct, Radix::Oct) => data.try_for_each(|(index, value)| {
                f.write_fmt(format_args!(
                    "{index:0oct_width$o} : {value:0oct_width$o}\n",
                ))
            })?,
            (Radix::Oct, Radix::Uns) => data.try_for_each(|(index, value)| {
                f.write_fmt(format_args!(
                    "{index:0oct_width$o} : {}\n",
                    value.unsigned()
                ))
            })?,

            (Radix::Dec | Radix::Uns, Radix::Bin) => data.try_for_each(|(index, value)| {
                f.write_fmt(format_args!("{index} : {value:0bin_width$b}\n"))
            })?,
            (Radix::Dec | Radix::Uns, Radix::Dec) => data
                .try_for_each(|(index, value)| f.write_fmt(format_args!("{index} : {value}\n")))?,
            (Radix::Dec | Radix::Uns, Radix::Hex) => data.try_for_each(|(index, value)| {
                f.write_fmt(format_args!("{index} : {value:0hex_width$X}\n"))
            })?,
            (Radix::Dec | Radix::Uns, Radix::Oct) => data.try_for_each(|(index, value)| {
                f.write_fmt(format_args!("{index} : {value:0oct_width$o}\n"))
            })?,
            (Radix::Dec | Radix::Uns, Radix::Uns) => data.try_for_each(|(index, value)| {
                f.write_fmt(format_args!("{index} : {}\n", value.unsigned()))
            })?,
        }

        f.write_str("END;")
    }
}

pub trait ToMif: Octal + Binary + UpperHex + Display {
    const WIDTH: usize;

    type Unsigned: Display;

    fn unsigned(&self) -> Self::Unsigned;
}

macro_rules! impl_to_mif {
    ($t:ty, $u:ty) => {
        impl ToMif for $t {
            type Unsigned = $u;

            const WIDTH: usize = Self::BITS as usize;

            fn unsigned(&self) -> Self::Unsigned {
                self.cast_unsigned()
            }
        }

        impl ToMif for $u {
            type Unsigned = $u;

            const WIDTH: usize = Self::BITS as usize;

            fn unsigned(&self) -> Self::Unsigned {
                *self
            }
        }
    };
}

impl_to_mif!(i8, u8);
impl_to_mif!(i16, u16);
impl_to_mif!(i32, u32);
impl_to_mif!(i64, u64);
impl_to_mif!(i128, u128);
impl_to_mif!(isize, usize);

#[cfg(target_family = "wasm")]
mod wasm {
    use paste::paste;
    use wasm_bindgen::prelude::*;

    use crate::Radix;

    #[wasm_bindgen]
    struct Mif;

    macro_rules! gen_writers {
        ($t:ty, $prefix:ident) => {
            paste! {
                #[wasm_bindgen]
                impl Mif {
                    #[wasm_bindgen(js_name = [<encode $prefix Array>])]
                    pub fn [<encode $t>](data: &[$t], address_radix: Radix, data_radix: Radix) -> String {
                        super::Mif::new(data, address_radix, data_radix).to_string()
                    }
                }
            }
        };
    }

    gen_writers!(u8, Uint8);
    gen_writers!(i8, Int8);
    gen_writers!(u16, Uint16);
    gen_writers!(i16, Int16);
    gen_writers!(u32, Uint32);
    gen_writers!(i32, Int32);
    gen_writers!(u64, Uint64);
    gen_writers!(i64, Int64);
}
