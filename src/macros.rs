use traits::*;

macro_rules! impl_as_bits {
    ($T:ty, $length:expr) => (
        impl AsBits for $T {
            fn as_bits(&self) -> Vec<bool> {
                // This is safe because the length is defined at compile time.
                unsafe {
                    self.as_bits_until_unchecked($length)
                }
            }

            fn as_bits_until(&self, until: usize) -> Vec<bool> {
                assert!(until <= $length);
                unsafe { self.as_bits_until_unchecked(until) }
            }

            unsafe fn as_bits_until_unchecked(&self, until: usize) -> Vec<bool> {
                let mut bits = Vec::<bool>::with_capacity(until);

                for i in 0..until {
                    // Select bit at index i.
                    //
                    // Let i = 3, x = 0110
                    // Create mask:
                    //      1 << 3 = 0100
                    // Select:
                    //      x & mask
                    //      0110 & 0100 = 0100
                    // Move back:
                    //      0100 >> 3 = 0001
                    // Boolean value : true
                    let bit = self & (1 << i);
                    bits.push(bit >> i == 1);
                }

                bits
            }
        }
    )
}

macro_rules! impl_from_bits {
    ($T:ty, $length:expr) => (
        impl FromBits for $T {
            fn from_bits(bits: &[bool]) -> Self {
                assert!(bits.len() <= $length);
                unsafe {
                    <$T>::from_bits_unchecked(bits)
                }
            }

            unsafe fn from_bits_unchecked(bits: &[bool]) -> Self {
                let mut val: $T = 0;

                for (i, &bit) in bits.iter().enumerate() {
                    // Push ones to the correct positions.
                    //
                    // Let i = 3, x = 0000
                    // Create mask:
                    //      1 << 3 = 0100
                    // Overwrite bit:
                    //      x | mask
                    //      0000 | 0100 = 0100
                    if bit {
                        val = val | (1 << i);
                    }
                }

                val
            }
        }
    )
}

impl_as_bits!(u8, 8);
impl_as_bits!(u16, 16);
impl_as_bits!(u32, 32);
impl_as_bits!(u64, 64);

impl_from_bits!(u8, 8);
impl_from_bits!(u16, 16);
impl_from_bits!(u32, 32);
impl_from_bits!(u64, 64);
