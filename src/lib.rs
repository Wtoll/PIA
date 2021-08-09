#![allow(incomplete_features)]
#![feature(const_generics)]
#![feature(const_evaluatable_checked)]

#![allow(unused_parens)]

//! # PIA (Packed Integer Array)
//!
//! PIA is a simple library for the Rust programming language that adds packed integer arrays for mass storage of oddly sized variables.
//!
//! To get started simply construct a new instance of a [`PackedIntegerArray`] with the desired amount of items and bits per item.
//! ```rust
//! // Constructs a new packed integer array with 5 bits per item and 4 items
//! let packed_array = pia::PackedIntegerArray::<5, 4>::new();
//! ```
//!
//! After that, use the array just like any other array. Items can be set using [`PackedIntegerArray::set()`],
//! items can be queried using [`PackedIntegerArray::get()`], and items can be reset back to 0 using [`PackedIntegerArray::clear()`].

extern crate log;
use log::warn;

#[cfg(feature = "serde")]
extern crate serde;
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

/// A helper function to determine the minimum amount of `u8`s that are needed in order to house `size` amount of items each of
/// `bits` amount of bits.
///
/// ```rust
/// // 4 items each with 3 bits per item is 12 bits in total which is housed by a minimum of 2 `u8`s
/// assert_eq!(pia::get_array_length(3, 4), 2);
/// ```
pub const fn get_array_length(bits: u8, size: usize) -> usize {
    (((bits as usize) * size) + (u8::BITS as usize) - 1) / (u8::BITS as usize)
}

/// A wrapped array that bit packs `LEN` amount of items each of `BITS` amount of bits into an array of `u8`s.
///
/// Use [`PackedIntegerArray::new()`] to construct a new instance.
///
/// ```rust
/// // Constructs a new packed integer array with 9 items and 3 bits per item
/// // All total this is wrapped array of 4 `u8`s because (9 * 3)/8 rounds up to 4
/// let mut packed_array = pia::PackedIntegerArray::<3, 9>::new(); 
///
/// packed_array.set(3, 7);
/// assert_eq!(packed_array.get(3), 7);
/// ```
#[derive(Debug, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct PackedIntegerArray<const BITS: u8, const LEN: usize>
where [u8; get_array_length(BITS, LEN)]: Sized {
    content: [u8; get_array_length(BITS, LEN)]
}

impl <const BITS: u8, const LEN: usize> PackedIntegerArray<BITS, LEN>
where [u8; get_array_length(BITS, LEN)]: Sized {
    /// Constructs a new packed integer array of `LEN` amount of items each of `BITS` amount of bits.
    ///
    /// ```rust
    /// // Constructs a new packed integer array with 9 items and 3 bits per item
    /// // All total this is wrapped array of 4 `u8`s because (9 * 3)/8 rounds up to 4
    /// let mut packed_array = pia::PackedIntegerArray::<3, 9>::new(); 
    ///
    /// packed_array.set(3, 7);
    /// assert_eq!(packed_array.get(3), 7);
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the packed integer value at the given index in the array.
    ///
    /// `index` references the index of the item in the array before bit-packing.
    ///
    /// ```rust
    /// let mut packed_array = pia::PackedIntegerArray::<3, 9>::new(); 
    ///
    /// // Sets the third value in the array to 7
    /// packed_array.set(3, 7);
    /// // Returns and test the third value in the array to check if it is correct
    /// assert_eq!(packed_array.get(3), 7);
    /// ```
    ///
    /// Note: just like a normal array, if an item outside of the array bounds is accessed the program will panic.
    pub fn get(&self, index: usize) -> u8 {
        if index >= LEN {
            panic!("index out of bounds: the len is {} but the index is {}", LEN, index);
        }

        let start_byte = (index * (BITS as usize)) / (u8::BITS as usize); // The index of the byte that contains the start of the item
        let start_bit = (index * (BITS as usize)) - (start_byte * (u8::BITS as usize)); // The first bit on that byte containing the start of the item

        let mut result = ((self.content[start_byte] << start_bit) >> ((u8::BITS as usize) - (BITS as usize)));
        if start_bit + (BITS as usize) > (u8::BITS as usize) {
            result |= (self.content[start_byte + 1] >> ((u8::BITS as usize * 2) - (start_bit + (BITS as usize))));
        }

        result
    }

    /// Sets the packed integer value at `index` in the array to `value`
    ///
    /// `index` references the index of the item in the array before bit-packing.
    ///
    /// ```rust
    /// let mut packed_array = pia::PackedIntegerArray::<3, 9>::new(); 
    ///
    /// // Sets the third value in the array to 7
    /// packed_array.set(3, 7);
    /// // Returns and test the third value in the array to check if it is correct
    /// assert_eq!(packed_array.get(3), 7);
    /// ```
    ///
    /// Note: just like a normal array, if an item outside of the array bounds is set the program will panic.
    ///
    /// Note: if the value passed is greater than the maximum value representable with the given amount of bits, the overflowing bits
    /// of greater significance are truncated.
    /// ```rust
    /// // Construct a packed integer array of bit size 3
    /// let mut packed_array = pia::PackedIntegerArray::<3, 9>::new();
    ///
    /// packed_array.set(2, 0b00000001);
    /// // Set item 4 to the same value as item 2 but with the fourth bit flipped
    /// // This will cause a warning in the log that the value may cause unintended functionality
    /// packed_array.set(4, 0b00001001);
    ///
    /// // When the values are returned they are the same because any bits greater than 3 are truncated
    /// assert_eq!(packed_array.get(2), packed_array.get(4));
    /// ```
    pub fn set(&mut self, index: usize, value: u8) {
        let max = usize::pow(2, BITS as u32);
        if value as usize >= max {
            warn!("Warning: input value {} is greater than the maximum value {} for {} bits. This may cause unintended functionality.", value, max - 1, BITS);
        }

        if index >= LEN {
            panic!("index out of bounds: the len is {} but the index is {}", LEN, index);
        }

        let start_byte = (index * (BITS as usize)) / (u8::BITS as usize); // The index of the byte that contains the start of the item
        let start_bit = (index * (BITS as usize)) - (start_byte * (u8::BITS as usize)); // The first bit on that byte containing the start of the item

        // Clear the current content
        if start_bit + (BITS as usize) > (u8::BITS as usize) {
            // If spread over multiple bytes
            self.content[start_byte] ^= ((self.content[start_byte] << start_bit) >> start_bit);
            self.content[start_byte + 1] ^= (self.content[start_byte + 1] >> ((u8::BITS as usize * 2) - (start_bit + (BITS as usize)))) << ((u8::BITS as usize * 2) - (start_bit + (BITS as usize)));
        } else {
            self.content[start_byte] ^= ((self.content[start_byte] << start_bit) >> ((u8::BITS as usize) - (BITS as usize))) << ((u8::BITS as usize) - (BITS as usize) - start_bit);
        }

        // Write the content
        self.content[start_byte] |= ((value << ((u8::BITS as usize) - (BITS as usize))) >> start_bit);
        if start_bit + (BITS as usize) > (u8::BITS as usize) {
            self.content[start_byte + 1] |= (value << (u8::BITS as usize * 2) - BITS as usize - start_bit);
        }
    }

    /// Sets the packed integer value at the given `index` in the array to 0
    ///
    /// `index` references the index of the item in the array before bit-packing.
    ///
    /// ```rust
    /// let mut packed_array = pia::PackedIntegerArray::<3, 9>::new(); 
    ///
    /// // Sets the third value in the array to 7
    /// packed_array.set(3, 7);
    /// // Clear that value
    /// packed_array.clear(3);
    /// // Returns and test the third value in the array to check if it is correct
    /// assert_eq!(packed_array.get(3), 0);
    /// ```
    ///
    /// Note: just like a normal array, if an item outside of the array bounds is set the program will panic.
    pub fn clear(&mut self, index: usize) {

        if index >= LEN {
            panic!("index out of bounds: the len is {} but the index is {}", LEN, index);
        }

        let start_byte = (index * (BITS as usize)) / (u8::BITS as usize); // The index of the byte that contains the start of the item
        let start_bit = (index * (BITS as usize)) - (start_byte * (u8::BITS as usize)); // The first bit on that byte containing the start of the item
        
        // Clear the current content
        if start_bit + (BITS as usize) > (u8::BITS as usize) {
            // If spread over multiple bytes
            self.content[start_byte] ^= ((self.content[start_byte] << start_bit) >> start_bit);
            self.content[start_byte + 1] ^= (self.content[start_byte + 1] >> ((u8::BITS as usize * 2) - (start_bit + (BITS as usize)))) << ((u8::BITS as usize * 2) - (start_bit + (BITS as usize)));
        } else {
            self.content[start_byte] ^= ((self.content[start_byte] << start_bit) >> ((u8::BITS as usize) - (BITS as usize))) << ((u8::BITS as usize) - (BITS as usize) - start_bit);
        }
    }

    /// Unpacks the packed array into an array of `u8`s
    ///
    /// ```rust
    /// let mut packed_array = pia::PackedIntegerArray::<3, 9>::new();
    /// packed_array.set(2, 4);
    /// packed_array.set(4, 5);
    /// assert_eq!(packed_array.unpack(), [0, 0, 4, 0, 5, 0, 0, 0, 0]);
    /// ```
    ///
    pub fn unpack(self) -> [u8; LEN] {
        let mut items: [u8; LEN] = [0; LEN];
        for i in 0..LEN {
            items[i] = self.get(i)
        }
        items
    }

}

use std::default::Default;
impl <const BITS: u8, const LEN: usize> Default for PackedIntegerArray<BITS, LEN>
where [u8; get_array_length(BITS, LEN)]: Sized {
    fn default() -> Self {
        Self {
            content: [0; get_array_length(BITS, LEN)]
        }
    }
}

use std::convert::AsMut;
impl <const BITS: u8, const LEN: usize> AsMut<[u8]> for PackedIntegerArray<BITS, LEN>
where [u8; get_array_length(BITS, LEN)]: Sized {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.content[..]
    }
}

use std::convert::AsRef;
impl <const BITS: u8, const LEN: usize> AsRef<[u8]> for PackedIntegerArray<BITS, LEN>
where [u8; get_array_length(BITS, LEN)]: Sized {
    fn as_ref(&self) -> &[u8] {
        &self.content[..]
    }
}

use std::hash::Hash;
use std::hash::Hasher;
impl <const BITS: u8, const LEN: usize> Hash for PackedIntegerArray<BITS, LEN>
where [u8; get_array_length(BITS, LEN)]: Sized {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        Hash::hash(&self.content[..], state)
    }
}

use std::iter::IntoIterator;
impl <const BITS: u8, const LEN: usize> IntoIterator for PackedIntegerArray<BITS, LEN>
where [u8; get_array_length(BITS, LEN)]: Sized {
    type Item = u8;
    type IntoIter = PackedIntegerArrayIterator<BITS, LEN>;

    fn into_iter(self) -> <Self as IntoIterator>::IntoIter {
        PackedIntegerArrayIterator {
            index: 0,
            array: self
        }
    }
}

use std::cmp::PartialEq;
impl <const BITS: u8, const LEN: usize> PartialEq<[u8; LEN]> for PackedIntegerArray<BITS, LEN>
where [u8; get_array_length(BITS, LEN)]: Sized {
    /// Determines whether this packed array has equivalent values to an array of `u8`s.
    ///
    /// The compared values are the "unpacked" values of the packed array.
    /// ```rust
    /// let mut packed_array = pia::PackedIntegerArray::<3, 9>::new();
    /// packed_array.set(2, 3);
    /// assert_eq!(packed_array, [0, 0, 3, 0, 0, 0, 0, 0, 0]);
    /// ```
    fn eq(&self, other: &[u8; LEN]) -> bool {
        for i in 0..LEN {
            if other[i] != self.get(i) {
                return false;
            }
        }
        return true;
    }
}

impl <const BITS: u8, const LEN: usize> PartialEq<PackedIntegerArray<BITS, LEN>> for PackedIntegerArray<BITS, LEN>
where [u8; get_array_length(BITS, LEN)]: Sized {
    fn eq(&self, other: &PackedIntegerArray<BITS, LEN>) -> bool {
        self.content == other.content
    }
}

use std::iter::Iterator;
/// A simple iterator that moves over every unpacked value in a [`PackedIntegerArray`].
///
/// ```rust
/// let mut packed_array = pia::PackedIntegerArray::<3, 9>::new();
/// 
/// packed_array.set(2, 5);
///
/// for item in packed_array {
///     println!("{}", item);
/// }
/// ```
pub struct PackedIntegerArrayIterator<const BITS: u8, const LEN: usize>
where [u8; get_array_length(BITS, LEN)]: Sized {
    index: usize,
    array: PackedIntegerArray<BITS, LEN>
}

impl <const BITS: u8, const LEN: usize> Iterator for PackedIntegerArrayIterator<BITS, LEN>
where [u8; get_array_length(BITS, LEN)]: Sized {
    type Item = u8;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if (self.index < LEN) {
            let val = self.array.get(self.index);
            self.index += 1;
            Some(val)
        } else {
            None
        }
    }
}