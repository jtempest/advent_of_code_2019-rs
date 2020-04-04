use num::{BigInt, Integer, ToPrimitive};
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ModNum {
    value: BigInt,
    modulo: BigInt,
}

impl ModNum {
    pub fn value(self) -> Option<u64> {
        self.value.to_u64()
    }

    pub fn big_value(self) -> BigInt {
        self.value
    }

    pub fn inv(self) -> ModNum {
        // assume we have a prime modulo and apply Fermat's little theorum
        ModNum {
            value: self.value.modpow(&(&self.modulo - 2), &self.modulo),
            modulo: self.modulo.clone(),
        }
    }

    fn ensure(&mut self) {
        self.value = self.value.mod_floor(&self.modulo);
    }
}

macro_rules! op {
    ($trait:ident, $method:ident) => {
        paste::item! {
            impl $trait<ModNum> for ModNum {
                type Output = ModNum;

                fn $method(self, other: ModNum) -> ModNum {
                    let mut result = self;
                    result.[<$method _assign>](other);
                    result
                }
            }

            impl [<$trait Assign>]<ModNum> for ModNum {
                fn [<$method _assign>](&mut self, other: ModNum) {
                    assert_eq!(self.modulo, other.modulo);
                    self.value.[<$method _assign>](other.value);
                    self.ensure();
                }
            }
        }
    };
}

op!(Add, add);
op!(Sub, sub);
op!(Mul, mul);

pub trait Modulo {
    fn modulo(self, modulo: u64) -> ModNum;
}

impl<T: Into<BigInt>> Modulo for T {
    fn modulo(self, modulo: u64) -> ModNum {
        //assert!(primes::is_prime(modulo));
        let modulo = modulo.into();
        let value = self.into().mod_floor(&modulo);
        ModNum { value, modulo }
    }
}
