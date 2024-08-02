use std::fmt::{Binary, Display, Octal, UpperHex, Write};

use crate::Radix;

pub struct Mif<'a, T> {
    address_radix: Radix,
    data_radix: Radix,
    data: &'a [T],
}

impl<'a, T> Mif<'a, T> {
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
        let address_radix = self.address_radix;
        let data_radix = self.data_radix;
        let mut data = self.data.iter().enumerate();

        f.write_fmt(format_args!("DEPTH = {depth}\nWIDTH = {width}\nADDRESS_RADIX = {address_radix}\nDATA_RADIX = {data_radix}\nCONTENT BEGIN\n"))?;

        match (address_radix, data_radix) {
            (Radix::Bin, Radix::Bin) => data.try_for_each(|(index, value)| {
                f.write_fmt(format_args!("{index:b} : {value:b}\n"))
            })?,
            (Radix::Bin, Radix::Dec) => data.try_for_each(|(index, value)| {
                f.write_fmt(format_args!("{index:b} : {value}\n"))
            })?,
            (Radix::Bin, Radix::Hex) => data.try_for_each(|(index, value)| {
                f.write_fmt(format_args!("{index:b} : {value:X}\n"))
            })?,
            (Radix::Bin, Radix::Oct) => data.try_for_each(|(index, value)| {
                f.write_fmt(format_args!("{index:b} : {value:o}\n"))
            })?,
            (Radix::Bin, Radix::Uns) => data.try_for_each(|(index, value)| {
                f.write_fmt(format_args!("{index:b} : {}\n", value.unsigned()))
            })?,

            (Radix::Hex, Radix::Bin) => data.try_for_each(|(index, value)| {
                f.write_fmt(format_args!("{index:X} : {value:b}\n"))
            })?,
            (Radix::Hex, Radix::Dec) => data.try_for_each(|(index, value)| {
                f.write_fmt(format_args!("{index:X} : {value}\n"))
            })?,
            (Radix::Hex, Radix::Hex) => data.try_for_each(|(index, value)| {
                f.write_fmt(format_args!("{index:X} : {value:X}\n"))
            })?,
            (Radix::Hex, Radix::Oct) => data.try_for_each(|(index, value)| {
                f.write_fmt(format_args!("{index:X} : {value:o}\n"))
            })?,
            (Radix::Hex, Radix::Uns) => data.try_for_each(|(index, value)| {
                f.write_fmt(format_args!("{index:X} : {}\n", value.unsigned()))
            })?,

            (Radix::Oct, Radix::Bin) => data.try_for_each(|(index, value)| {
                f.write_fmt(format_args!("{index:o} : {value}\n"))
            })?,
            (Radix::Oct, Radix::Dec) => data.try_for_each(|(index, value)| {
                f.write_fmt(format_args!("{index:o} : {value:b}\n"))
            })?,
            (Radix::Oct, Radix::Hex) => data.try_for_each(|(index, value)| {
                f.write_fmt(format_args!("{index:o} : {value:X}\n"))
            })?,
            (Radix::Oct, Radix::Oct) => data.try_for_each(|(index, value)| {
                f.write_fmt(format_args!("{index:o} : {value:o}\n"))
            })?,
            (Radix::Oct, Radix::Uns) => data.try_for_each(|(index, value)| {
                f.write_fmt(format_args!("{index:o} : {}\n", value.unsigned()))
            })?,

            (Radix::Dec | Radix::Uns, Radix::Bin) => data.try_for_each(|(index, value)| {
                f.write_fmt(format_args!("{index} : {value:b}\n"))
            })?,
            (Radix::Dec | Radix::Uns, Radix::Dec) => data
                .try_for_each(|(index, value)| f.write_fmt(format_args!("{index} : {value}\n")))?,
            (Radix::Dec | Radix::Uns, Radix::Hex) => data.try_for_each(|(index, value)| {
                f.write_fmt(format_args!("{index} : {value:X}\n"))
            })?,
            (Radix::Dec | Radix::Uns, Radix::Oct) => data.try_for_each(|(index, value)| {
                f.write_fmt(format_args!("{index} : {value:o}\n"))
            })?,
            (Radix::Dec | Radix::Uns, Radix::Uns) => data.try_for_each(|(index, value)| {
                f.write_fmt(format_args!("{index} : {}\n", value.unsigned()))
            })?,
        }

        f.write_str("END;")
    }
}

trait ToMif: Octal + Binary + UpperHex + Display {
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

mod test {
    use super::*;

    #[test]
    fn i8_hex_uns() {
        let data = &[-2i8, 2, -3, -1, 2, 0];
        let writer = Mif::new(data, Radix::Hex, Radix::Uns);
        assert_eq!(format!("{writer}"), "DEPTH = 6\nWIDTH = 8\nADDRESS_RADIX = HEX\nDATA_RADIX = UNS\nCONTENT BEGIN\n0 : 254\n1 : 2\n2 : 253\n3 : 255\n4 : 2\n5 : 0\nEND;")
    }

    #[test]
    fn i16_bin_dec() {
        let data = &[-2i16, 2, -3, -1, 2, 0];
        let writer = Mif::new(data, Radix::Bin, Radix::Dec);
        assert_eq!(format!("{writer}"), "DEPTH = 6\nWIDTH = 16\nADDRESS_RADIX = BIN\nDATA_RADIX = DEC\nCONTENT BEGIN\n0 : -2\n1 : 2\n10 : -3\n11 : -1\n100 : 2\n101 : 0\nEND;")
    }
}
