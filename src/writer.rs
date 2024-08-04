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

macro_rules! write_data {
    ($f:expr, $data:expr, $index:expr, $value:expr, signed) => {{
        let Some(first) = $data.next() else {
            return Ok(());
        };

        let group = $data.try_fold(
            GroupState {
                start: 0,
                length: 1,
                value: first.1.signed(),
            },
            |mut acc, (index, value)| {
                let value = value.signed();

                if acc.value == value {
                    acc.length += 1;
                } else {
                    if index - acc.start > 1 {
                        write!($f, "[");
                        write!($f, $index, acc.start);
                        write!($f, "..");
                        write!($f, $index, index - 1);
                        write!($f, "]:");
                    } else {
                        write!($f, $index, acc.start);
                        write!($f, ":");
                    }

                    write!($f, $value, acc.value);
                    write!($f, ";\n");

                    acc.start = index;
                    acc.value = value;
                    acc.length = 1;
                }
                Ok(acc)
            },
        )?;

        if group.length > 1 {
            write!($f, "[");
            write!($f, $index, group.start);
            write!($f, "..");
            write!($f, $index, group.start + group.length - 1);
            write!($f, "]:");
        } else {
            write!($f, $index, group.start);
            write!($f, ":");
        }

        write!($f, $value, group.value);
        write!($f, ";\n");
    }};
    ($f:expr, $data:expr, $index:expr, $value:expr) => {{
        let Some(first) = $data.next() else {
            return Ok(());
        };

        let group = $data.try_fold(
            GroupState {
                start: 0,
                length: 1,
                value: first.1.bits(),
            },
            |mut acc, (index, value)| {
                let value = value.bits();

                if acc.value == value {
                    acc.length += 1;
                } else {
                    if index - acc.start > 1 {
                        write!($f, "[");
                        write!($f, $index, acc.start);
                        write!($f, "..");
                        write!($f, $index, index - 1);
                        write!($f, "]:");
                    } else {
                        write!($f, $index, acc.start);
                        write!($f, ":");
                    }

                    write!($f, $value, acc.value);
                    write!($f, ";\n");

                    acc.start = index;
                    acc.value = value;
                    acc.length = 1;
                }
                Ok(acc)
            },
        )?;

        if group.length > 1 {
            write!($f, "[");
            write!($f, $index, group.start);
            write!($f, "..");
            write!($f, $index, group.start + group.length - 1);
            write!($f, "]:");
        } else {
            write!($f, $index, group.start);
            write!($f, ":");
        }

        write!($f, $value, group.value);
        write!($f, ";\n");
    }};
}

struct GroupState<T: PartialEq> {
    start: usize,
    length: usize,
    value: T,
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

        f.write_fmt(format_args!("DEPTH={depth};\nWIDTH={width};\nADDRESS_RADIX={address_radix};\nDATA_RADIX={data_radix};\nCONTENT BEGIN\n"))?;

        match (address_radix, data_radix) {
            (Radix::Bin, Radix::Bin) => {
                write_data!(f, data, "{:0bin_width$b}", "{:0bin_width$b}")
            }
            (Radix::Bin, Radix::Dec) => {
                write_data!(f, data, "{:0bin_width$b}", "{}", signed)
            }
            (Radix::Bin, Radix::Hex) => {
                write_data!(f, data, "{:0bin_width$b}", "{:0hex_width$X}")
            }
            (Radix::Bin, Radix::Oct) => {
                write_data!(f, data, "{:0bin_width$b}", "{:0oct_width$o}")
            }
            (Radix::Bin, Radix::Uns) => write_data!(f, data, "{:0bin_width$b}", "{}"),

            (Radix::Hex, Radix::Bin) => {
                write_data!(f, data, "{:0hex_width$X}", "{:0bin_width$b}")
            }
            (Radix::Hex, Radix::Dec) => {
                write_data!(f, data, "{:0hex_width$X}", "{}", signed)
            }
            (Radix::Hex, Radix::Hex) => {
                write_data!(f, data, "{:0hex_width$X}", "{:0hex_width$X}")
            }
            (Radix::Hex, Radix::Oct) => {
                write_data!(f, data, "{:0hex_width$X}", "{:0oct_width$o}")
            }
            (Radix::Hex, Radix::Uns) => write_data!(f, data, "{:0hex_width$X}", "{}"),

            (Radix::Oct, Radix::Bin) => {
                write_data!(f, data, "{:0oct_width$o}", "{:0bin_width$b}")
            }
            (Radix::Oct, Radix::Dec) => {
                write_data!(f, data, "{:0oct_width$o}", "{}", signed)
            }
            (Radix::Oct, Radix::Hex) => {
                write_data!(f, data, "{:0oct_width$o}", "{:0hex_width$X}")
            }
            (Radix::Oct, Radix::Oct) => {
                write_data!(f, data, "{:0oct_width$o}", "{:0oct_width$o}")
            }
            (Radix::Oct, Radix::Uns) => write_data!(f, data, "{:0oct_width$o}", "{}"),

            (Radix::Dec | Radix::Uns, Radix::Bin) => {
                write_data!(f, data, "{}", "{:0bin_width$b}")
            }
            (Radix::Dec | Radix::Uns, Radix::Dec) => {
                write_data!(f, data, "{}", "{}", signed)
            }
            (Radix::Dec | Radix::Uns, Radix::Hex) => {
                write_data!(f, data, "{}", "{:0hex_width$X}")
            }
            (Radix::Dec | Radix::Uns, Radix::Oct) => {
                write_data!(f, data, "{}", "{:0oct_width$o}")
            }
            (Radix::Dec | Radix::Uns, Radix::Uns) => write_data!(f, data, "{}", "{}"),
        }

        f.write_str("END;")
    }
}

pub trait ToMif {
    const WIDTH: usize;

    type SignedBinary: Display + PartialEq;
    type Binary: Octal + Binary + UpperHex + Display + PartialEq;

    fn signed(&self) -> Self::SignedBinary;
    fn bits(&self) -> Self::Binary;
}

macro_rules! impl_to_mif {
    ($s:ty, $u:ty, $f:ty) => {
        impl_to_mif!($s, $u);

        impl ToMif for $f {
            type Binary = $u;
            type SignedBinary = $u;

            const WIDTH: usize = <$u>::BITS as usize;

            fn signed(&self) -> Self::SignedBinary {
                self.to_bits()
            }

            fn bits(&self) -> Self::Binary {
                self.to_bits()
            }
        }
    };
    ($s:ty, $u:ty) => {
        impl ToMif for $s {
            type Binary = $u;
            type SignedBinary = $s;

            const WIDTH: usize = <$u>::BITS as usize;

            fn signed(&self) -> Self::SignedBinary {
                *self
            }

            fn bits(&self) -> Self::Binary {
                self.cast_unsigned()
            }
        }

        impl ToMif for $u {
            type Binary = $u;
            type SignedBinary = $u;

            const WIDTH: usize = <$u>::BITS as usize;

            fn signed(&self) -> Self::SignedBinary {
                *self
            }

            fn bits(&self) -> Self::Binary {
                *self
            }
        }

        impl ToMif for std::num::NonZero<$s> {
            type Binary = $u;
            type SignedBinary = $s;

            const WIDTH: usize = <$u>::BITS as usize;

            fn signed(&self) -> Self::SignedBinary {
                self.get()
            }

            fn bits(&self) -> Self::Binary {
                self.get().cast_unsigned()
            }
        }

        impl ToMif for std::num::NonZero<$u> {
            type Binary = $u;
            type SignedBinary = $u;

            const WIDTH: usize = <$u>::BITS as usize;

            fn signed(&self) -> Self::SignedBinary {
                self.get()
            }

            fn bits(&self) -> Self::Binary {
                self.get()
            }
        }
    };
}

impl_to_mif!(i8, u8);
#[cfg(feature = "f16")]
impl_to_mif!(i16, u16, f16);
#[cfg(not(feature = "f16"))]
impl_to_mif!(i16, u16);
impl_to_mif!(i32, u32, f32);
impl_to_mif!(i64, u64, f64);
#[cfg(feature = "f128")]
impl_to_mif!(i128, u128, f128);
#[cfg(not(feature = "f128"))]
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
