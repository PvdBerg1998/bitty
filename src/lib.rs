//! # bitty - helps a bit
//! `bitty` contains functions to extract bits from, and put back into integer types.
//!
//! ## Usage
//! Include `bitty` in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! bitty = "1.0"
//! ```
//!
//! # Use cases
//! Extracting individual bits:
//!
//! ```rust
//! # use bitty::*;
//! let five_as_bits: Vec<bool> = 5u8.as_bits();
//! // Expected: 1 0 1 0 0 0 0 0
//! assert_eq!(five_as_bits, vec![true, false, true, false, false, false, false, false]);
//! ```
//!
//! Putting bits back into an u8:
//!
//! ```rust
//! # use bitty::*;
//! let five_from_bits: u8 = u8::from_bits(&5u8.as_bits());
//! assert_eq!(5, five_from_bits);
//! ```
//!
//! Creating an integer from some bits:
//!
//! ```rust
//! # use bitty::*;
//! // Note that the vector does not have to contain 64 bits.
//! // Missing bits default to 0.
//! let one_from_bits: u64 = u64::from_bits(&vec![true]);
//! assert_eq!(1, one_from_bits);
//! ```

pub use traits::*;

mod macros;

pub mod traits {
    pub trait AsBits {
        /// Extracts all bits as a boolean vector.
        ///
        /// # Examples
        /// ```rust
        /// # use bitty::*;
        /// let bits: Vec<bool> = 5u8.as_bits();
        /// // Expected: 1 0 1 0 0 0 0 0
        /// assert_eq!(bits, vec![true, false, true, false, false, false, false, false]);
        /// ```
        fn as_bits(&self) -> Vec<bool>;

        /// Extracts bits until an index as a boolean vector.
        ///
        /// # Arguments
        /// * `until` - Take bits until this index (exclusive)
        ///
        /// # Panics
        /// This function panics if `until` is smaller than the size of the integer type.
        ///
        /// # Examples
        /// ```rust
        /// # use bitty::*;
        /// let bits: Vec<bool> = 5u64.as_bits_until(4);
        /// // Expected: 1 0 1 0
        /// assert_eq!(bits, vec![true, false, true, false]);
        /// ```
        fn as_bits_until(&self, until: usize) -> Vec<bool>;

        /// Extracts bits until an index as a boolean vector.
        ///
        /// # Arguments
        /// * `until` - Take bits until this index (exclusive)
        ///
        /// # Safety
        /// This function does not check if `until` is larger than the bits in the integer type.
        /// The code might try to bit shift outside the size of the integer type, which is UB.
        ///
        /// # Examples
        /// ```rust
        /// # use bitty::*;
        /// unsafe {
        ///     let bits: Vec<bool> = 5u64.as_bits_until_unchecked(4);
        ///     // Expected: 1 0 1 0
        ///     assert_eq!(bits, vec![true, false, true, false]);
        /// }
        /// ```
        unsafe fn as_bits_until_unchecked(&self, until: usize) -> Vec<bool>;
    }

    pub trait FromBits {
        /// Puts bits back into an integer type.
        ///
        /// # Arguments
        /// * `bits` - A boolean slice containing all bits of the integer.
        /// Missing bits default to 0.
        ///
        /// # Panics
        /// This function panics if the length of `bits` is larger than the size of the integer type.
        ///
        /// # Examples
        /// ```rust
        /// # use bitty::*;
        /// let bits: Vec<bool> = vec![true, true, true, true];
        /// assert_eq!(15, u8::from_bits(&bits));
        /// ```
        fn from_bits(bits: &[bool]) -> Self;

        /// Puts bits back into an integer type.
        ///
        /// # Arguments
        /// * `bits` - A boolean slice containing all bits of the integer.
        /// Missing bits default to 0.
        ///
        /// # Safety
        /// The code might try to bit shift outside the size of the integer type, which is UB.
        /// This might cause a panic at runtime.
        ///
        /// # Examples
        /// ```rust
        /// # use bitty::*;
        /// unsafe {
        ///     let bits: Vec<bool> = vec![true, true, true, true];
        ///     assert_eq!(15, u8::from_bits_unchecked(&bits));
        /// }
        /// ```
        unsafe fn from_bits_unchecked(bits: &[bool]) -> Self;
    }
}
