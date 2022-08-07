const MANTISSA_BITS_64: u64 = 0x000f_ffff_ffff_ffff;
const EXPONENT_BITS_64: u64 = 0x7ff0_0000_0000_0000;
const SIGN_BIT_64: u64 = 0x8000_0000_0000_0000;
const UNIT_64: u64 = 0x0000_0000_0000_0001;
const EXPONENT_UNIT_64: u64 = MANTISSA_BITS_64 + UNIT_64;

const MANTISSA_BITS_32: u32 = 0x007f_ffff;
const EXPONENT_BITS_32: u32 = 0x7f80_0000;
const SIGN_BIT_32: u32 = 0x8000_0000;
const UNIT_32: u32 = 0x0000_0001;
const EXPONENT_UNIT_32: u32 = MANTISSA_BITS_32 + UNIT_32;

pub trait FloatOps<T> {
    fn mantissa(self) -> T;
    fn exponent(self) -> T;
    fn next(self) -> Self;
}

impl FloatOps<u64> for f64 {
    fn mantissa(self) -> u64 {
        self.to_bits() & MANTISSA_BITS_64 as u64
    }
    fn exponent(self) -> u64 {
        self.to_bits() & EXPONENT_BITS_64 as u64
    }
    fn next(self) -> f64 {
        if self.mantissa() == MANTISSA_BITS_64 as u64 {
            f64::from_bits(self as u64 & SIGN_BIT_64 | (self.exponent() + EXPONENT_UNIT_64))
        } else {
            f64::from_bits(
                self as u64 & SIGN_BIT_64 | self.exponent() | (self.mantissa() + UNIT_64),
            )
        }
    }
}

impl FloatOps<u32> for f32 {
    fn mantissa(self) -> u32 {
        self.to_bits() & MANTISSA_BITS_32 as u32
    }
    fn exponent(self) -> u32 {
        self.to_bits() & EXPONENT_BITS_32 as u32
    }
    fn next(self) -> f32 {
        if self.mantissa() == MANTISSA_BITS_32 as u32 {
            f32::from_bits(self as u32 & SIGN_BIT_32 | (self.exponent() + EXPONENT_UNIT_32))
        } else {
            f32::from_bits(
                self as u32 & SIGN_BIT_32 | self.exponent() | (self.mantissa() + UNIT_32),
            )
        }
    }
}

// pub fn next_64(x: f64) -> f64 {
//   let shift_mantissa = f64::MANTISSA_DIGITS;
//   let bits = x.to_bits();
//   let mantissa = bits & 0b00000000_00011111_11111111_11111111_11111111_11111111_11111111_11111111;
//   let exponent = (bits >> f64::MANTISSA_DIGITS) & 0x7FF;
//   let sign = (bits >> 63) & 1;
//   let next_mantissa = mantissa + 1;
//   let next_bits = (next_mantissa << shift_mantissa) | (exponent << shift_mantissa) | sign;
//   println!("bits: {}", bits);
//   println!("next_bits: {}", next_bits);
//   f64::from_bits(next_bits)
// }

#[cfg(test)]
mod tests {
    use super::*;

    // f64 tests
    #[test]
    fn test_mantissa_64_1() {
        assert_eq!(1f64.mantissa(), 0);
    }

    #[test]
    fn test_mantissa_64_2() {
        assert_eq!(256f64.mantissa(), 0);
    }

    #[test]
    fn test_mantissa_64_3() {
        assert_eq!(0.125f64.mantissa(), 0);
    }

    #[test]
    fn test_mantissa_64_4() {
        assert_eq!((1.0 + f64::EPSILON).mantissa(), 1);
    }

    #[test]
    fn test_mantissa_64_5() {
        assert_eq!((1.0 + 123.0 * f64::EPSILON).mantissa(), 123);
    }

    #[test]
    fn test_mantissa_64_6() {
        let n = 0x000eeeeeeeeeeeee as u64;
        assert_eq!((1.0 + n as f64 * f64::EPSILON).mantissa(), n);
    }

    #[test]
    fn test_exponent_64_1() {
        assert_eq!(f64::INFINITY.exponent(), 0x7ff0_0000_0000_0000);
    }

    #[test]
    fn test_exponent_64_2() {
        assert_eq!(1f64.exponent(), 0x3ff0_0000_0000_0000);
    }

    #[test]
    fn test_exponent_64_3() {
        assert_eq!((-1f64).exponent(), 0x3ff0_0000_0000_0000);
    }

    #[test]
    fn test_next_64_1() {
        assert_eq!(1f64.next(), 1.0 + f64::EPSILON);
    }

    // f32 tests
    #[test]
    fn test_mantissa_32_1() {
        assert_eq!(1f32.mantissa(), 0);
    }

    #[test]
    fn test_mantissa_32_2() {
        assert_eq!(256f32.mantissa(), 0);
    }

    #[test]
    fn test_mantissa_32_3() {
        assert_eq!(0.125f32.mantissa(), 0);
    }

    #[test]
    fn test_mantissa_32_4() {
        assert_eq!((1.0 + f32::EPSILON).mantissa(), 1);
    }

    #[test]
    fn test_mantissa_32_5() {
        assert_eq!((1.0 + 123.0 * f32::EPSILON).mantissa(), 123);
    }

    #[test]
    fn test_mantissa_32_6() {
        let n = 0x00777777 as u32;
        assert_eq!((1.0 + n as f32 * f32::EPSILON).mantissa(), n);
    }

    #[test]
    fn test_exponent_32_1() {
        assert_eq!(f32::INFINITY.exponent(), 0x7f80_0000);
    }

    #[test]
    fn test_exponent_32_2() {
        assert_eq!(1f32.exponent(), 0x3f80_0000);
    }

    #[test]
    fn test_exponent_32_3() {
        assert_eq!((-1f32).exponent(), 0x3f80_0000);
    }

    #[test]
    fn test_next_32_1() {
        assert_eq!(1f32.next(), 1.0 + f32::EPSILON);
    }
}
