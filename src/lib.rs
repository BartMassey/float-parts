//! # IEEE floating-point representation stuff
//! Bart Massey 2024
//!
//! Many thanks to Keith Packard and Mike Haertel for
//! helping to work out this representation.

/// Get the "parts" of an IEEE floating-point number.
pub trait ToFloatParts : Copy {
    /// Number of significant binary digits for the type,
    /// including the implicit 1 bit. For example, 24 for
    /// `f32`.
    const NUM_SIG_BITS: u32;

    /// Number of binary digits for the exponent type. For
    /// example, 8 for `f32`.
    const NUM_EXP_BITS: u32;

    /// Exponent denoting Inf and NaN values.
    const EXP_INF_NAN: Self::Exp;

    /// Type for integer representation of the
    /// mantissa/significand. This should be an unsigned
    /// integer type.
    ///
    /// "Mantissa" is deprecated, IEEE uses "significand"
    /// which is horrible. So "sigbits" it is.
    type SigBits;
    /// Type for integer representation of the
    /// exponent. This should be a signed integer type large
    /// enough to contain an "adjusted" representation of
    /// the exponent.
    type Exp;

    /// Given a float, return the sigbits, exponent and
    /// sign.
    /// 
    /// The sigbits will be right-justified, with the binary
    /// point at position [NUM_SIG_BITS] - 1 from the right.
    /// The implicit 1 for non-denorms will be made explicit.
    ///
    /// The exp will be relative to the binary point.
    ///
    /// The sign will be +1 for positive or -1 for negative.
    ///
    /// It is defined behavior to call this on an infinity
    /// or NaN. That said, the results may take a bit of
    /// interpretation to be useful, as the sigbits will
    /// contain an extra 1 bit and the exponent will have
    /// been adjusted.
    fn to_float_parts(self) -> (Self::SigBits, Self::Exp, i8);
}

impl ToFloatParts for f32 {
    const NUM_SIG_BITS: u32 = f32::MANTISSA_DIGITS;
    // Why is there no constant for this?
    const NUM_EXP_BITS: u32 = 8;
    const EXP_INF_NAN: i16 = 255 - 127;
    type SigBits = u32;
    type Exp = i16;
    
    fn to_float_parts(self) -> (Self::SigBits, Self::Exp, i8) {
        let bits = self.to_bits();
        let sign = 1 - ((bits >> 30) & 2) as i8;
        let mut exp = ((bits >> 23) & 0xff) as i16;
        let is_denorm = exp == 0;
        exp += -127 + 23 - is_denorm as i16;
        let mut sigbits = bits & ((1 << 23) - 1);
        sigbits |= !(is_denorm as u32) << 24;
        (sigbits, exp, sign)
    }
}
