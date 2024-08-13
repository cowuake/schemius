use core::fmt;
use std::{
    any::TypeId,
    cmp,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use num::{integer::Roots, BigInt, BigRational, Complex, One, ToPrimitive, Zero};

#[cfg(not(feature = "i64"))]
#[cfg(not(feature = "i128"))]
pub type NativeInt = i32;
#[cfg(feature = "i64")]
pub type NativeInt = i64;
#[cfg(feature = "i128")]
pub type NativeInt = i128;

#[cfg(not(feature = "f64"))]
pub type NativeFloat = f32;
#[cfg(feature = "f64")]
pub type NativeFloat = f64;

pub type NativeBigInt = BigInt;
pub type NativeRational = BigRational;
pub type NativeComplex = Complex<NativeFloat>;

#[derive(Debug, Clone)]
pub enum SNumber {
    Int(NativeInt),
    BigInt(NativeBigInt),
    Rational(NativeRational),
    Float(NativeFloat),
    Complex(NativeComplex),
}

pub struct NumericalConstant;

impl NumericalConstant {
    pub const AVOGADRO: SNumber = SNumber::Float(6.0221515e23);
    pub const BOLTZMANN: SNumber = SNumber::Float(1.380650e23);
    pub const EULER: SNumber = SNumber::Float(2.718281828459045);
    pub const GOLDEN_RATIO: SNumber = SNumber::Float(1.618033988749895);
    pub const GRAVITATIONAL_CONSTANT: SNumber = SNumber::Float(6.67300e-11);
    pub const PI: SNumber = SNumber::Float(3.141592653589793);
    pub const PLANCK: SNumber = SNumber::Float(6.626068e-34);
}

pub trait NativeCasts {
    fn to_float(&self) -> Option<NativeFloat>;
}

impl NativeCasts for NativeBigInt {
    fn to_float(&self) -> Option<NativeFloat> {
        if TypeId::of::<NativeFloat>() == TypeId::of::<f32>() {
            self.to_f32().map(|f| f as NativeFloat)
        } else if TypeId::of::<NativeFloat>() == TypeId::of::<f64>() {
            self.to_f64().map(|f| f as NativeFloat)
        } else {
            panic!("Lacking proper implementation for NativeFloat")
        }
    }
}

impl NativeCasts for NativeRational {
    fn to_float(&self) -> Option<NativeFloat> {
        if TypeId::of::<NativeFloat>() == TypeId::of::<f32>() {
            Some(self.to_f32().unwrap() as NativeFloat)
        } else if TypeId::of::<NativeFloat>() == TypeId::of::<f64>() {
            Some(self.to_f64().unwrap() as NativeFloat)
        } else {
            panic!("Lacking proper implementation for NativeFloat")
        }
    }
}

impl fmt::Display for SNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SNumber::Int(ref val) => write!(f, "{}", val),
            SNumber::BigInt(ref val) => write!(f, "{}", val),
            SNumber::Rational(ref val) => write!(f, "{}", val),
            SNumber::Float(ref val) => write!(f, "{:?}", val),
            SNumber::Complex(ref val) => {
                let mut sign_text = "";
                if val.im >= 0.0 {
                    sign_text = "+";
                }
                write!(f, "{}{}{}i", val.re, sign_text, val.im)
            }
        }
    }
}

macro_rules! impl_ref_op {
    ($($capital:ident, $small:ident, $op:tt, $limit:expr)*) => {
    $(
        impl $capital for &SNumber {
            type Output = SNumber;

            fn $small(self, other: Self) -> Self::Output {
                match self {
                    SNumber::Int(l) => match other {
                        SNumber::Int(r) => if cmp::max((if *l < 0 { l + 1 } else { *l }).abs(), (if *r < 0 { r + 1 } else { *r }).abs()) < $limit { SNumber::Int(l $op r) } else { SNumber::BigInt(NativeBigInt::from(*l) $op NativeBigInt::from(*r)) },
                        SNumber::BigInt(r) => SNumber::BigInt(l $op r),
                        SNumber::Rational(r) => SNumber::Rational(NativeRational::from(NativeBigInt::from(*l)) $op r),
                        SNumber::Float(r) => SNumber::Float(*l as NativeFloat $op r),
                        SNumber::Complex(c) => SNumber::Complex(NativeComplex::from(*l as NativeFloat) $op c),
                    },
                    SNumber::BigInt(l) => match other {
                        SNumber::Int(r) => SNumber::BigInt(l.clone() as NativeBigInt $op r),
                        SNumber::BigInt(r) => SNumber::BigInt(l $op r),
                        SNumber::Rational(r) => SNumber::Rational(NativeRational::from(l.clone()) $op r),
                        SNumber::Float(r) => SNumber::Float(l.to_float().unwrap() $op r),
                        SNumber::Complex(c) => SNumber::Complex(NativeComplex::from(l.to_float().unwrap()) $op c),
                    },
                    SNumber::Rational(l) => match other {
                        SNumber::Int(r) => SNumber::Rational(l $op NativeBigInt::from(*r)),
                        SNumber::BigInt(r) => SNumber::Rational(l $op r),
                        SNumber::Rational(r) => SNumber::Rational(l $op r),
                        SNumber::Float(r) => SNumber::Float(l.to_float().unwrap() $op r),
                        SNumber::Complex(c) => SNumber::Complex(NativeComplex::from(l.to_float().unwrap() as NativeFloat) $op c),
                    },
                    SNumber::Float(l) => match other {
                        SNumber::Int(r) => SNumber::Float(l $op *r as NativeFloat),
                        SNumber::BigInt(r) => SNumber::Float(l $op r.to_float().unwrap()),
                        SNumber::Rational(r) => SNumber::Float(l $op r.to_float().unwrap()),
                        SNumber::Float(r) => SNumber::Float(l $op r),
                        SNumber::Complex(c) => SNumber::Complex(NativeComplex::from(*l) $op c),
                    },
                    SNumber::Complex(l) => match other {
                        SNumber::Int(r) => SNumber::Complex(l $op NativeComplex::new(*r as NativeFloat, 0.0)),
                        SNumber::BigInt(r) => SNumber::Complex(l $op NativeComplex::new(r.to_float().unwrap(), 0.0)),
                        SNumber::Rational(r) => SNumber::Complex(l $op NativeComplex::new(r.to_float().unwrap(), 0.0)),
                        SNumber::Float(r) => SNumber::Complex(l $op NativeComplex::new(*r, 0.0)),
                        SNumber::Complex(r) => SNumber::Complex(l $op r),
                    }
                }
            }
        }
    )*}
}

impl_ref_op! {
    Add, add, +, NativeInt::MAX / 2
    Sub, sub, -, NativeInt::MAX / 2
    Mul, mul, *, (NativeInt::MAX / 2).sqrt()
}

impl Div for &SNumber {
    type Output = SNumber;

    fn div(self, other: Self) -> Self::Output {
        match self {
            SNumber::Int(l) => match other {
                SNumber::Int(r) => SNumber::Rational(NativeRational::new(
                    NativeBigInt::from(*l),
                    NativeBigInt::from(*r),
                )),
                SNumber::BigInt(r) => {
                    SNumber::Rational(NativeRational::new(NativeBigInt::from(*l), r.clone()))
                }
                SNumber::Rational(r) => {
                    SNumber::Rational(NativeRational::from(NativeBigInt::from(*l)) / r)
                }
                SNumber::Float(r) => SNumber::Float(*l as NativeFloat / r),
                SNumber::Complex(c) => SNumber::Complex(NativeComplex::from(*l as NativeFloat) / c),
            },
            SNumber::BigInt(l) => match other {
                SNumber::Int(r) => {
                    SNumber::Rational(NativeRational::new(l.clone(), NativeBigInt::from(*r)))
                }
                SNumber::BigInt(r) => SNumber::Rational(NativeRational::new(l.clone(), r.clone())),
                SNumber::Rational(r) => SNumber::Rational(NativeRational::from(l.clone()) / r),
                SNumber::Float(r) => SNumber::Float(l.to_float().unwrap() / r),
                SNumber::Complex(c) => {
                    SNumber::Complex(NativeComplex::from(l.to_float().unwrap()) / c)
                }
            },
            SNumber::Rational(l) => match other {
                SNumber::Int(r) => SNumber::Rational(l / NativeBigInt::from(*r)),
                SNumber::BigInt(r) => SNumber::Rational(l / r),
                SNumber::Rational(r) => SNumber::Rational(l / r),
                SNumber::Float(r) => SNumber::Float(l.to_float().unwrap() / r),
                SNumber::Complex(c) => {
                    SNumber::Complex(NativeComplex::from(l.to_float().unwrap() as NativeFloat) / c)
                }
            },
            SNumber::Float(l) => match other {
                SNumber::Int(r) => SNumber::Float(l / *r as NativeFloat),
                SNumber::BigInt(r) => SNumber::Float(l / r.to_float().unwrap()),
                SNumber::Rational(r) => SNumber::Float(l / r.to_float().unwrap()),
                SNumber::Float(r) => SNumber::Float(l / r),
                SNumber::Complex(c) => SNumber::Complex(NativeComplex::from(*l) / c),
            },
            SNumber::Complex(l) => match other {
                SNumber::Int(r) => SNumber::Complex(l / NativeComplex::new(*r as NativeFloat, 0.0)),
                SNumber::BigInt(r) => {
                    SNumber::Complex(l / NativeComplex::new(r.to_float().unwrap(), 0.0))
                }
                SNumber::Rational(r) => {
                    SNumber::Complex(l / NativeComplex::new(r.to_float().unwrap(), 0.0))
                }
                SNumber::Float(r) => SNumber::Complex(l / NativeComplex::new(*r, 0.0)),
                SNumber::Complex(r) => SNumber::Complex(l / r),
            },
        }
    }
}

macro_rules! impl_partial_eq_ord_op {
    ($($fn:ident: $op:tt)*) => {
    $(
        fn $fn(&self, other: &Self) -> bool {
            match self {
                SNumber::Int(l) => match other {
                    SNumber::Int(r) => l $op r,
                    SNumber::BigInt(r) => NativeBigInt::from(*l) $op *r,
                    SNumber::Rational(r) => NativeRational::new(NativeBigInt::from(*l), NativeBigInt::from(1 as NativeInt)) $op *r,
                    SNumber::Float(r) => (*l as NativeFloat) $op *r,
                    SNumber::Complex(_) => false,
                },
                SNumber::BigInt(l) => match other {
                    SNumber::Int(r) => *l $op NativeBigInt::from(*r),
                    SNumber::BigInt(r) => l $op r,
                    SNumber::Rational(r) => NativeRational::new(l.clone(), NativeBigInt::from(1 as NativeInt)) $op *r,
                    SNumber::Float(r) => l.to_float().unwrap() $op *r,
                    SNumber::Complex(_) => false,
                },
                SNumber::Rational(l) => match other {
                    SNumber::Int(r) => *l $op NativeRational::new(NativeBigInt::from(*r), NativeBigInt::from(1 as NativeInt)),
                    SNumber::BigInt(r) => *l $op NativeRational::new(r.clone(), NativeBigInt::from(1 as NativeInt)),
                    SNumber::Rational(r) => l $op r,
                    SNumber::Float(r) => l.to_float().unwrap() $op *r,
                    SNumber::Complex(_) => false,
                },
                SNumber::Float(l) => match other {
                    SNumber::Int(r) => *l $op *r as NativeFloat,
                    SNumber::BigInt(r) => *l $op r.to_float().unwrap(),
                    SNumber::Rational(r) => *l $op r.to_float().unwrap(),
                    SNumber::Float(r) => l $op r,
                    SNumber::Complex(_) => false,
                },
                SNumber::Complex(l) => match other {
                    SNumber::Complex(r) => l.re $op r.re && l.im $op r.im,
                    _ => false,
                }
            }
        }
    )*}
}

impl PartialEq for SNumber {
    impl_partial_eq_ord_op! {
        eq: ==
        // ne: != // Not needed (hint by Clippy)
    }
}

impl PartialOrd for SNumber {
    impl_partial_eq_ord_op! {
        ge: >=
        gt: >
        le: <=
        lt: <
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self {
            SNumber::Int(l) => match other {
                SNumber::Int(r) => Some(l.cmp(r)),
                SNumber::BigInt(r) => Some(NativeBigInt::from(*l).cmp(r)),
                SNumber::Rational(r) => Some(
                    NativeRational::new(NativeBigInt::from(*l), NativeBigInt::from(1 as NativeInt))
                        .cmp(r),
                ),
                SNumber::Float(r) => Some(
                    NativeRational::new(NativeBigInt::from(*l), NativeBigInt::from(1 as NativeInt))
                        .cmp(&NativeRational::from_float(*r).unwrap()),
                ),
                SNumber::Complex(_) => None,
            },
            SNumber::BigInt(l) => match other {
                SNumber::Int(r) => Some((*l).cmp(&NativeBigInt::from(*r))),
                SNumber::BigInt(r) => Some(l.cmp(r)),
                SNumber::Rational(r) => {
                    Some(NativeRational::new(l.clone(), NativeBigInt::from(1 as NativeInt)).cmp(r))
                }
                SNumber::Float(r) => Some(
                    NativeRational::new(l.clone(), NativeBigInt::from(1 as NativeInt))
                        .cmp(&NativeRational::from_float(*r).unwrap()),
                ),
                SNumber::Complex(_) => None,
            },
            SNumber::Rational(l) => match other {
                SNumber::Int(r) => Some(l.cmp(&NativeRational::new(
                    NativeBigInt::from(*r),
                    NativeBigInt::from(1 as NativeInt),
                ))),
                SNumber::BigInt(r) => Some(
                    (*l).cmp(&NativeRational::new(r.clone(), NativeBigInt::from(1 as NativeInt))),
                ),
                SNumber::Rational(r) => Some(l.cmp(r)),
                SNumber::Float(r) => Some(l.cmp(&NativeRational::from_float(*r).unwrap())),
                SNumber::Complex(_) => None,
            },
            SNumber::Float(l) => match other {
                SNumber::Int(r) => {
                    Some((NativeRational::from_float(*l).unwrap()).cmp(&NativeRational::new(
                        NativeBigInt::from(*r),
                        NativeBigInt::from(1 as NativeInt),
                    )))
                }
                SNumber::BigInt(r) => Some(
                    (NativeRational::from_float(*l).unwrap())
                        .cmp(&NativeRational::new(r.clone(), NativeBigInt::from(1 as NativeInt))),
                ),
                SNumber::Rational(r) => Some((&NativeRational::from_float(*l).unwrap()).cmp(r)),
                SNumber::Float(r) => Some(
                    (NativeRational::from_float(*l).unwrap())
                        .cmp(&NativeRational::from_float(*r).unwrap()),
                ),
                SNumber::Complex(_) => None,
            },
            SNumber::Complex(_) => None,
        }
    }
}

macro_rules! impl_op {
    ($($small:ident, $capital:ident)*) => {
    $(
        impl $capital for SNumber {
            type Output = SNumber;

            fn $small(self, rhs: Self) -> Self::Output {
                (&self).$small(&rhs)
            }
        }
    )*}
}

impl_op! {
    add, Add
    sub, Sub
    mul, Mul
    div, Div
}

macro_rules! impl_op_assign {
    ($($small:ident, $capital:ident, $op:ident)*) => {
    $(
        impl $capital for SNumber {
            fn $small(&mut self, rhs: Self) {
                *self = (&*self).$op(&rhs)
            }
        }
    )*}
}

impl_op_assign! {
    add_assign, AddAssign, add
    sub_assign, SubAssign, sub
    mul_assign, MulAssign, mul
    div_assign, DivAssign, div
}

impl SNumber {
    pub fn is_exact(&self) -> bool {
        match self {
            SNumber::Int(_) => true,
            SNumber::BigInt(_) => true,
            SNumber::Rational(_) => true,
            SNumber::Float(_) => false,
            SNumber::Complex(c) => {
                SNumber::Float(c.re).is_exact() && SNumber::Float(c.im).is_exact()
            }
        }
    }

    pub fn to_int(&self) -> Result<NativeInt, String> {
        match self {
            SNumber::Int(internal) => Ok(*internal),
            other => Err(format!("Exception: {} is not a proper integer", other)),
        }
    }

    pub fn is_integer(&self) -> bool {
        match self {
            SNumber::Int(_) => true,
            SNumber::BigInt(_) => true,
            SNumber::Rational(r) => r.denom().is_one(),
            SNumber::Float(r) => r.fract() == 0.0,
            SNumber::Complex(c) => {
                if c.im.is_zero() {
                    SNumber::Float(c.re).is_integer()
                } else {
                    false
                }
            }
        }
    }

    pub fn is_real(&self) -> bool {
        match self {
            SNumber::Int(_) => true,
            SNumber::BigInt(_) => true,
            SNumber::Rational(_) => true,
            SNumber::Float(_) => true,
            SNumber::Complex(c) => {
                if c.im.is_zero() {
                    SNumber::Float(c.re).is_real()
                } else {
                    false
                }
            }
        }
    }

    pub fn is_rational(&self) -> bool {
        match self {
            SNumber::Int(_) => true,
            SNumber::BigInt(_) => true,
            SNumber::Rational(_) => true,
            SNumber::Float(number) => match number {
                number if number.is_infinite() => false,
                number if number.is_nan() => false,
                _ => true,
            },
            SNumber::Complex(_) => false,
        }
    }

    pub fn is_complex(&self) -> bool {
        match self {
            SNumber::Int(_) => true,
            SNumber::BigInt(_) => true,
            SNumber::Rational(_) => true,
            SNumber::Float(_) => true,
            SNumber::Complex(_) => true,
        }
    }

    pub fn is_zero(&self) -> bool {
        // TODO: Generalize
        match self {
            SNumber::Int(number) => *number == 0,
            SNumber::BigInt(number) => number.is_zero(),
            SNumber::Rational(number) => number.is_zero(),
            SNumber::Float(number) => *number == 0.0,
            SNumber::Complex(number) => number == &Complex::zero(),
        }
    }

    pub fn is_infinite(&self) -> bool {
        match self {
            SNumber::Float(number) => number.is_infinite(),
            _ => false,
        }
    }

    pub fn is_nan(&self) -> bool {
        match self {
            SNumber::Float(number) => number.is_nan(),
            _ => false,
        }
    }
}

#[cfg(test)]
pub mod tests {
    use num::bigint::ToBigInt;

    use crate::core::s_expression::s_number::{
        NativeBigInt, NativeComplex, NativeFloat, NativeInt, NativeRational,
    };

    use super::SNumber;

    macro_rules! snumber_op_test {
        ($($fn:ident: {operator: $op:tt, lhs: $lhs:expr, rhs: $rhs:expr, expected: $expected:expr};)*) => {
        $(
            #[test]
            fn $fn() {
                let lhs = $lhs;
                let rhs = $rhs;
                let res = &lhs $op &rhs;
                let expected = $expected;

                assert_eq!(res, expected)
            }
        )*}
    }

    snumber_op_test! {
        snumber_op_int_int_add: {
            operator: +,
            lhs: SNumber::Int(1),
            rhs: SNumber::Int(2),
            expected: SNumber::Int(3)
        };
        snumber_op_int_int_sub: {
            operator: -,
            lhs: SNumber::Int(1),
            rhs: SNumber::Int(2),
            expected: SNumber::Int(-1)
        };
        snumber_op_int_int_mul: {
            operator: *,
            lhs: SNumber::Int(1),
            rhs: SNumber::Int(2),
            expected: SNumber::Int(2)
        };
        snumber_op_int_int_div: {
            operator: /,
            lhs: SNumber::Int(1),
            rhs: SNumber::Int(2),
            expected: SNumber::Rational("1/2".parse::<NativeRational>().unwrap())
        };
        snumber_op_int_int_eq: {
            operator: ==,
            lhs: SNumber::Int(1),
            rhs: SNumber::Int(2),
            expected: false
        };
        snumber_op_int_int_gt: {
            operator: >,
            lhs: SNumber::Int(1),
            rhs: SNumber::Int(2),
            expected: false
        };
        snumber_op_int_int_lt: {
            operator: <,
            lhs: SNumber::Int(1),
            rhs: SNumber::Int(2),
            expected: true
        };
        snumber_op_int_bigint_add: {
            operator: +,
            lhs: SNumber::Int(1),
            rhs: SNumber::BigInt("77777777777777777777".parse::<NativeBigInt>().unwrap()),
            expected: SNumber::BigInt("77777777777777777778".parse::<NativeBigInt>().unwrap())
        };
        snumber_op_int_bigint_sub: {
            operator: -,
            lhs: SNumber::Int(1),
            rhs: SNumber::BigInt("77777777777777777777".parse::<NativeBigInt>().unwrap()),
            expected: SNumber::BigInt("-77777777777777777776".parse::<NativeBigInt>().unwrap())
        };
        snumber_op_int_bigint_mul: {
            operator: *,
            lhs: SNumber::Int(1),
            rhs: SNumber::BigInt("77777777777777777777".parse::<NativeBigInt>().unwrap()),
            expected: SNumber::BigInt("77777777777777777777".parse::<NativeBigInt>().unwrap())
        };
        snumber_op_int_bigint_div: {
            operator: /,
            lhs: SNumber::Int(1),
            rhs: SNumber::BigInt("77777777777777777777".parse::<NativeBigInt>().unwrap()),
            expected: SNumber::Rational(NativeRational::new((1 as NativeInt).to_bigint().unwrap(), "77777777777777777777".parse::<NativeBigInt>().unwrap()))
        };
        snumber_op_int_bigint_eq: {
            operator: ==,
            lhs: SNumber::Int(1),
            rhs: SNumber::BigInt("1".parse::<NativeBigInt>().unwrap()),
            expected: true
        };
        snumber_op_int_bigint_gt: {
            operator: >,
            lhs: SNumber::Int(2),
            rhs: SNumber::BigInt("1".parse::<NativeBigInt>().unwrap()),
            expected: true
        };
        snumber_op_int_bigint_lt: {
            operator: <,
            lhs: SNumber::Int(2),
            rhs: SNumber::BigInt("1".parse::<NativeBigInt>().unwrap()),
            expected: false
        };
        snumber_op_int_rational_add: {
            operator: +,
            lhs: SNumber::Int(1),
            rhs: SNumber::Rational("22222222222222222222/2".parse::<NativeRational>().unwrap()),
            expected: SNumber::Rational("11111111111111111112".parse::<NativeRational>().unwrap())
        };
        snumber_op_int_rational_sub: {
            operator: -,
            lhs: SNumber::Int(1),
            rhs: SNumber::Rational("22222222222222222222/2".parse::<NativeRational>().unwrap()),
            expected: SNumber::Rational("-11111111111111111110".parse::<NativeRational>().unwrap())
        };
        snumber_op_int_rational_mul: {
            operator: *,
            lhs: SNumber::Int(3),
            rhs: SNumber::Rational("22222222222222222222/2".parse::<NativeRational>().unwrap()),
            expected: SNumber::Rational("33333333333333333333".parse::<NativeRational>().unwrap())
        };
        snumber_op_int_rational_div: {
            operator: /,
            lhs: SNumber::Int(3),
            rhs: SNumber::Rational("1/2".parse::<NativeRational>().unwrap()),
            expected: SNumber::Rational("6".parse::<NativeRational>().unwrap())
        };
        snumber_op_int_rational_eq: {
            operator: ==,
            lhs: SNumber::Int(3),
            rhs: SNumber::Rational("6/2".parse::<NativeRational>().unwrap()),
            expected: true
        };
        snumber_op_int_rational_gt: {
            operator: >,
            lhs: SNumber::Int(3),
            rhs: SNumber::Rational("5/2".parse::<NativeRational>().unwrap()),
            expected: true
        };
        snumber_op_int_rational_lt: {
            operator: <,
            lhs: SNumber::Int(3),
            rhs: SNumber::Rational("7/2".parse::<NativeRational>().unwrap()),
            expected: true
        };
        snumber_op_int_float_add: {
            operator: +,
            lhs: SNumber::Int(1),
            rhs: SNumber::Float(0.1),
            expected: SNumber::Float(1.1)
        };
        snumber_op_int_float_sub: {
            operator: -,
            lhs: SNumber::Int(1),
            rhs: SNumber::Float(0.1),
            expected: SNumber::Float(0.9)
        };
        snumber_op_int_float_mul: {
            operator: *,
            lhs: SNumber::Int(2),
            rhs: SNumber::Float(2.0),
            expected: SNumber::Float(4.0)
        };
        snumber_op_int_float_div: {
            operator: /,
            lhs: SNumber::Int(2),
            rhs: SNumber::Float(2.0),
            expected: SNumber::Float(1.0)
        };
        snumber_op_int_float_eq: {
            operator: ==,
            lhs: SNumber::Int(2),
            rhs: SNumber::Float(2.0),
            expected: true
        };
        snumber_op_int_float_gt: {
            operator: >,
            lhs: SNumber::Int(3),
            rhs: SNumber::Float(2.0),
            expected: true
        };
        snumber_op_int_float_lt: {
            operator: <,
            lhs: SNumber::Int(1),
            rhs: SNumber::Float(2.0),
            expected: true
        };
        snumber_op_bigint_int_add: {
            operator: +,
            lhs: SNumber::BigInt("77777777777777777777".parse::<NativeBigInt>().unwrap()),
            rhs: SNumber::Int(1),
            expected: SNumber::BigInt("77777777777777777778".parse::<NativeBigInt>().unwrap())
        };
        snumber_op_bigint_int_sub: {
            operator: -,
            lhs: SNumber::BigInt("77777777777777777777".parse::<NativeBigInt>().unwrap()),
            rhs: SNumber::Int(1),
            expected: SNumber::BigInt("77777777777777777776".parse::<NativeBigInt>().unwrap())
        };
        snumber_op_bigint_int_mul: {
            operator: *,
            lhs: SNumber::BigInt("77777777777777777777".parse::<NativeBigInt>().unwrap()),
            rhs: SNumber::Int(1),
            expected: SNumber::BigInt("77777777777777777777".parse::<NativeBigInt>().unwrap())
        };
        snumber_op_bigint_int_div: {
            operator: /,
            lhs: SNumber::BigInt("77777777777777777777".parse::<NativeBigInt>().unwrap()),
            rhs: SNumber::Int(7),
            expected: SNumber::BigInt("11111111111111111111".parse::<NativeBigInt>().unwrap())
        };
        snumber_op_bigint_int_eq: {
            operator: ==,
            lhs: SNumber::BigInt(NativeBigInt::from(7 as NativeInt)),
            rhs: SNumber::Int(7),
            expected: true
        };
        snumber_op_bigint_int_gt: {
            operator: >,
            lhs: SNumber::BigInt(NativeBigInt::from(7 as NativeInt)),
            rhs: SNumber::Int(8),
            expected: false
        };
        snumber_op_bigint_int_lt: {
            operator: <,
            lhs: SNumber::BigInt(NativeBigInt::from(7 as NativeInt)),
            rhs: SNumber::Int(8),
            expected: true
        };
        snumber_op_bigint_bigint_add: {
            operator: +,
            lhs: SNumber::BigInt("11111111111111111111".parse::<NativeBigInt>().unwrap()),
            rhs: SNumber::BigInt("22222222222222222222".parse::<NativeBigInt>().unwrap()),
            expected: SNumber::BigInt("33333333333333333333".parse::<NativeBigInt>().unwrap())
        };
        snumber_op_bigint_bigint_sub: {
            operator: -,
            lhs: SNumber::BigInt("-11111111111111111111".parse::<NativeBigInt>().unwrap()),
            rhs: SNumber::BigInt("-22222222222222222222".parse::<NativeBigInt>().unwrap()),
            expected: SNumber::BigInt("11111111111111111111".parse::<NativeBigInt>().unwrap())
        };
        snumber_op_bigint_bigint_mul: {
            operator: *,
            lhs: SNumber::BigInt("20000000000000000000".parse::<NativeBigInt>().unwrap()),
            rhs: SNumber::BigInt("60000000000000000000".parse::<NativeBigInt>().unwrap()),
            expected: SNumber::BigInt("1200000000000000000000000000000000000000".parse::<NativeBigInt>().unwrap())
        };
        snumber_op_bigint_bigint_div: {
            operator: /,
            lhs: SNumber::BigInt(NativeBigInt::from(7 as NativeInt)),
            rhs: SNumber::BigInt("77777777777777777777".parse::<NativeBigInt>().unwrap()),
            expected: SNumber::Rational("7/77777777777777777777".parse::<NativeRational>().unwrap())
        };
        snumber_op_bigint_bigint_eq: {
            operator: ==,
            lhs: SNumber::BigInt("77777777777777777777".parse::<NativeBigInt>().unwrap()),
            rhs: SNumber::BigInt("77777777777777777777".parse::<NativeBigInt>().unwrap()),
            expected: true
        };
        snumber_op_bigint_bigint_gt: {
            operator: >,
            lhs: SNumber::BigInt("-77777777777777777777".parse::<NativeBigInt>().unwrap()),
            rhs: SNumber::BigInt("77777777777777777777".parse::<NativeBigInt>().unwrap()),
            expected: false
        };
        snumber_op_bigint_bigint_lt: {
            operator: <,
            lhs: SNumber::BigInt("-77777777777777777777".parse::<NativeBigInt>().unwrap()),
            rhs: SNumber::BigInt("77777777777777777777".parse::<NativeBigInt>().unwrap()),
            expected: true
        };
        snumber_op_bigint_rational_add: {
            operator: +,
            lhs: SNumber::BigInt("11111111111111111111".parse::<NativeBigInt>().unwrap()),
            rhs: SNumber::Rational("66666666666666666666/2".parse::<NativeRational>().unwrap()),
            expected: SNumber::Rational("44444444444444444444".parse::<NativeRational>().unwrap())
        };
        snumber_op_bigint_rational_sub: {
            operator: -,
            lhs: SNumber::BigInt("11111111111111111111".parse::<NativeBigInt>().unwrap()),
            rhs: SNumber::Rational("66666666666666666666/2".parse::<NativeRational>().unwrap()),
            expected: SNumber::Rational("-22222222222222222222".parse::<NativeRational>().unwrap())
        };
        snumber_op_bigint_rational_mul: {
            operator: *,
            lhs: SNumber::BigInt("20000000000000000000".parse::<NativeBigInt>().unwrap()),
            rhs: SNumber::Rational("50000000000000000000/2".parse::<NativeRational>().unwrap()),
            expected: SNumber::Rational("500000000000000000000000000000000000000".parse::<NativeRational>().unwrap())
        };
        snumber_op_bigint_rational_eq: {
            operator: ==,
            lhs: SNumber::BigInt("20000000000000000000".parse::<NativeBigInt>().unwrap()),
            rhs: SNumber::Rational("40000000000000000000/2".parse::<NativeRational>().unwrap()),
            expected: true
        };
        snumber_op_bigint_rational_gt: {
            operator: >,
            lhs: SNumber::BigInt("20000000000000000000".parse::<NativeBigInt>().unwrap()),
            rhs: SNumber::Rational("50000000000000000000/2".parse::<NativeRational>().unwrap()),
            expected: false
        };
        snumber_op_bigint_rational_lt: {
            operator: <,
            lhs: SNumber::BigInt("20000000000000000000".parse::<NativeBigInt>().unwrap()),
            rhs: SNumber::Rational("50000000000000000000/2".parse::<NativeRational>().unwrap()),
            expected: true
        };
        snumber_op_bigint_float_add: {
            operator: +,
            lhs: SNumber::BigInt("9999999999999999999".parse::<NativeBigInt>().unwrap()),
            rhs: SNumber::Float(1.0),
            expected: SNumber::Float("10000000000000000000".parse::<NativeFloat>().unwrap())
        };
        snumber_op_bigint_float_sub: {
            operator: -,
            lhs: SNumber::BigInt("9999999999999999999".parse::<NativeBigInt>().unwrap()),
            rhs: SNumber::Float(-2.0),
            expected: SNumber::Float("10000000000000000001".parse::<NativeFloat>().unwrap())
        };
        snumber_op_bigint_float_mul: {
            operator: *,
            lhs: SNumber::BigInt("-2222222222222222222".parse::<NativeBigInt>().unwrap()),
            rhs: SNumber::Float(2.0),
            expected: SNumber::Float("-4444444444444444444".parse::<NativeFloat>().unwrap())
        };
        snumber_op_bigint_float_div: {
            operator: /,
            lhs: SNumber::BigInt("-2222222222222222222".parse::<NativeBigInt>().unwrap()),
            rhs: SNumber::Float(2.0),
            expected: SNumber::Float("-1111111111111111111".parse::<NativeFloat>().unwrap())
        };
        snumber_op_bigint_float_eq: {
            operator: ==,
            lhs: SNumber::BigInt("-2222222222222222222".parse::<NativeBigInt>().unwrap()),
            rhs: SNumber::Float(-2222222222222222222.0),
            expected: true
        };
        snumber_op_rational_int_sub: {
            operator: -,
            lhs: SNumber::Rational("-22222222222222222222/2".parse::<NativeRational>().unwrap()),
            rhs: SNumber::Int(1),
            expected: SNumber::Rational("-11111111111111111112".parse::<NativeRational>().unwrap())
        };
        snumber_op_rational_int_mul: {
            operator: *,
            lhs: SNumber::Rational("-33333333333333333333/2".parse::<NativeRational>().unwrap()),
            rhs: SNumber::Int(4),
            expected: SNumber::Rational("-66666666666666666666".parse::<NativeRational>().unwrap())
        };
        snumber_op_rational_int_div: {
            operator: /,
            lhs: SNumber::Rational("-33333333333333333333/2".parse::<NativeRational>().unwrap()),
            rhs: SNumber::Int(-40),
            expected: SNumber::Rational("33333333333333333333/80".parse::<NativeRational>().unwrap())
        };
        snumber_op_rational_int_eq: {
            operator: ==,
            lhs: SNumber::Rational("-12/4".parse::<NativeRational>().unwrap()),
            rhs: SNumber::Int(-3),
            expected: true
        };
        snumber_op_rational_int_gt: {
            operator: >,
            lhs: SNumber::Rational("-12/4".parse::<NativeRational>().unwrap()),
            rhs: SNumber::Int(-4),
            expected: true
        };
        snumber_op_rational_int_lt: {
            operator: <,
            lhs: SNumber::Rational("-12/4".parse::<NativeRational>().unwrap()),
            rhs: SNumber::Int(-4),
            expected: false
        };
        snumber_op_rational_bigint_add: {
            operator: +,
            lhs: SNumber::Rational("66666666666666666666/2".parse::<NativeRational>().unwrap()),
            rhs: SNumber::BigInt("11111111111111111111".parse::<NativeBigInt>().unwrap()),
            expected: SNumber::Rational("44444444444444444444".parse::<NativeRational>().unwrap())
        };
        snumber_op_rational_float_add: {
            operator: +,
            lhs: SNumber::Rational("1/2".parse::<NativeRational>().unwrap()),
            rhs: SNumber::Float(0.5),
            expected: SNumber::Float(1.0)
        };
        snumber_op_rational_rational_add: {
            operator: +,
            lhs: SNumber::Rational("1/6".parse::<NativeRational>().unwrap()),
            rhs: SNumber::Rational("1/6".parse::<NativeRational>().unwrap()),
            expected: SNumber::Rational("1/3".parse::<NativeRational>().unwrap())
        };
        snumber_op_float_int_add: {
            operator: +,
            lhs: SNumber::Float(0.1),
            rhs: SNumber::Int(1),
            expected: SNumber::Float(1.1)
        };
        snumber_op_float_bigint_add: {
            operator: +,
            lhs: SNumber::Float(1.0),
            rhs: SNumber::BigInt("9999999999999999999".parse::<NativeBigInt>().unwrap()),
            expected: SNumber::Float("10000000000000000000".parse::<NativeFloat>().unwrap())
        };
        snumber_op_float_rational_add: {
            operator: +,
            lhs: SNumber::Float(0.5),
            rhs: SNumber::Rational("1/2".parse::<NativeRational>().unwrap()),
            expected: SNumber::Float(1.0)
        };
        snumber_op_float_float_add: {
            operator: +,
            lhs: SNumber::Float(0.5),
            rhs: SNumber::Float(1.5),
            expected: SNumber::Float(2.0)
        };
        snumber_op_float_float_sub: {
            operator: -,
            lhs: SNumber::Float(0.5),
            rhs: SNumber::Float(1.5),
            expected: SNumber::Float(-1.0)
        };
        snumber_op_float_float_mul: {
            operator: *,
            lhs: SNumber::Float(2.0),
            rhs: SNumber::Float(10.0),
            expected: SNumber::Float(20.0)
        };
        snumber_op_float_float_div: {
            operator: /,
            lhs: SNumber::Float(10.0),
            rhs: SNumber::Float(2.0),
            expected: SNumber::Float(5.0)
        };
        snumber_op_float_float_eq: {
            operator: ==,
            lhs: SNumber::Float(0.1),
            rhs: SNumber::Float(0.1),
            expected: true
        };
        snumber_op_int_complex_add: {
            operator: +,
            lhs: SNumber::Int(1),
            rhs: SNumber::Complex(2.0.into()),
            expected: SNumber::Complex(3.0.into())
        };
        snumber_op_int_complex_sub: {
            operator: -,
            lhs: SNumber::Int(1),
            rhs: SNumber::Complex(2.0.into()),
            expected: SNumber::Complex((-1.0).into())
        };
        snumber_op_int_complex_mul: {
            operator: *,
            lhs: SNumber::Int(1),
            rhs: SNumber::Complex(2.0.into()),
            expected: SNumber::Complex(2.0.into())
        };
        snumber_op_int_complex_div: {
            operator: /,
            lhs: SNumber::Int(1),
            rhs: SNumber::Complex(NativeComplex::new(2.0, 1.0)),
            expected: SNumber::Complex(NativeComplex::new(0.4, -0.2))
        };
        snumber_op_complex_complex_eq: {
            operator: ==,
            lhs: SNumber::Complex(NativeComplex::new(2.0, 1.0)),
            rhs: SNumber::Complex(NativeComplex::new(2.0, 1.0)),
            expected: true
        };
        snumber_op_int_complex_gt: { // TODO: Handle differently
            operator: >,
            lhs: SNumber::Complex(1.0.into()),
            rhs: SNumber::Int(2),
            expected: false
        };
        snumber_op_int_complex_lt: { // TODO: Handle differently
            operator: <,
            lhs: SNumber::Int(1),
            rhs: SNumber::Complex(2.0.into()),
            expected: false
        };
    }

    #[test]
    fn snumber_promotion_int_add() {
        let max = NativeInt::MAX;
        let res = SNumber::Int(max) + SNumber::Int(max);

        assert_eq!(res, SNumber::BigInt(NativeBigInt::from(max) + NativeBigInt::from(max)))
    }

    #[test]
    fn snumber_promotion_int_sub() {
        let min = NativeInt::MIN;
        let max = NativeInt::MAX;
        let res = SNumber::Int(min) - SNumber::Int(max);

        assert_eq!(res, SNumber::BigInt(NativeBigInt::from(min) - NativeBigInt::from(max)))
    }

    #[test]
    fn snumber_promotion_int_mul() {
        let max = NativeInt::MAX;
        let res = SNumber::Int(max) * SNumber::Int(max);

        assert_eq!(res, SNumber::BigInt(NativeBigInt::from(max) * NativeBigInt::from(max)))
    }
}
