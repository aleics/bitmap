use std::ops::{BitAnd, BitOr, Not};

/// Bitmap stores a bitmap in chunks of 64 bits
#[derive(Debug, PartialEq, Eq)]
pub struct Bitmap {
    chunks: Vec<usize>,
    size: usize,
}

impl Bitmap {
    /// Create a new `Bitmap` with a fixed size.
    pub fn new(size: usize) -> Self {
        Bitmap {
            chunks: Bitmap::chunks_with_size(size),
            size,
        }
    }

    /// Get the bit value from a given position
    pub fn get(&self, position: usize) -> bool {
        let (chunk_index, bit_index_in_chunk) = bit_index(position, usize::BITS as usize);
        let chunk = self.chunks[chunk_index];

        let position_bit = 1 << bit_index_in_chunk;

        position_bit == (position_bit & chunk)
    }

    /// Set a bit value in a given position
    pub fn set(&mut self, position: usize, value: u8) {
        if position >= self.size {
            panic!("Index out of bounds");
        }

        let (chunk_index, bit_index_in_chunk) = bit_index(position, usize::BITS as usize);

        if value == 1 {
            self.set_one(chunk_index, bit_index_in_chunk)
        } else if value == 0 {
            self.set_zero(chunk_index, bit_index_in_chunk)
        } else {
            panic!("Invalid bit value {value}")
        }
    }

    /// Sets a 1 to the corresponding chunk and bit position.
    ///
    /// This operation can be implemented by the following steps:
    ///   * Left shift "bit" times a 1
    ///   * Apply a bitwise OR with the existing chunk value
    ///
    /// Example: Set 1 in position 1 for "00101"
    ///   * Left shift 1, 1 times: "10"
    ///   * OR: "00101" OR "00010" = "00111"
    fn set_one(&mut self, chunk: usize, bit: usize) {
        self.chunks[chunk] |= 1 << bit
    }

    /// Sets a 0 to the corresponding chunk and bit position.
    ///
    /// This operation can be implemented by the following steps:
    ///   * Left shift "bit" times a 1
    ///   * Apply a bitwise NOT to the previous step
    ///   * Apply a bitwise AND operator with the existing chunk value
    ///
    /// Example: Set 0 in position 2 for "00101"
    ///   * Left shift 1, 2 times: "100"
    ///   * Bitwise NOT of previous: "011"
    ///   * AND: "00101" AND "00011" = "00001"
    fn set_zero(&mut self, chunk: usize, bit: usize) {
        self.chunks[chunk] &= !(1 << bit)
    }

    fn chunks_with_size(size: usize) -> Vec<usize> {
        vec![0; chunks_count(size, usize::BITS as usize)]
    }
}

impl BitAnd for &Bitmap {
    type Output = Bitmap;

    fn bitand(self, rhs: Self) -> Self::Output {
        let size = self.size.min(rhs.size);
        let mut chunks = Bitmap::chunks_with_size(size);

        for (id, chunk) in chunks.iter_mut().enumerate() {
            *chunk = self.chunks[id] & rhs.chunks[id];
        }

        Bitmap { chunks, size }
    }
}

impl BitOr for &Bitmap {
    type Output = Bitmap;

    fn bitor(self, rhs: Self) -> Self::Output {
        let size = self.size.min(rhs.size);
        let mut chunks = Bitmap::chunks_with_size(size);

        for (id, chunk) in chunks.iter_mut().enumerate() {
            *chunk = self.chunks[id] | rhs.chunks[id];
        }

        Bitmap { chunks, size }
    }
}

impl Not for &Bitmap {
    type Output = Bitmap;

    fn not(self) -> Self::Output {
        let chunks = self.chunks.iter().map(|chunk| !chunk).collect();

        Bitmap {
            chunks,
            size: self.size,
        }
    }
}

impl From<&str> for Bitmap {
    fn from(value: &str) -> Self {
        let mut bitmap = Bitmap::new(value.len());

        for (index, char) in value.chars().rev().enumerate() {
            match char {
                '1' => bitmap.set(index, 1),
                '0' => bitmap.set(index, 0),
                _ => panic!("Can not convert from string slice. Unexpected character {char}"),
            }
        }

        bitmap
    }
}

/// Calculate the amount of chunks needed for the desired bitmap size, and the bits per chunk.
fn chunks_count(size: usize, chunk_bit_size: usize) -> usize {
    (size + chunk_bit_size - 1) / chunk_bit_size
}

/// Calculate the bit index in the chunks by a give position, and chunk bit size.
fn bit_index(bit_index: usize, chunk_bit_size: usize) -> (usize, usize) {
    let chunk_index = bit_index / chunk_bit_size;
    let bit_index_in_chunk = bit_index % chunk_bit_size;

    (chunk_index, bit_index_in_chunk)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunks_of_size() {
        assert_eq!(0, chunks_count(0, 16));
        assert_eq!(1, chunks_count(1, 16));
        assert_eq!(1, chunks_count(15, 16));
        assert_eq!(1, chunks_count(16, 16));
        assert_eq!(2, chunks_count(17, 16));
    }

    #[test]
    fn test_get() {
        let bitmap = Bitmap::from("10101");
        assert!(bitmap.get(2));
        assert_eq!(bitmap.get(1), false);
    }

    #[test]
    fn test_or() {
        let first = Bitmap::from("00001");
        let second = Bitmap::from("00010");

        assert_eq!(&first | &second, Bitmap::from("00011"));
    }

    #[test]
    fn test_and() {
        let first = Bitmap::from("00011");
        let second = Bitmap::from("00010");

        assert_eq!(&first & &second, Bitmap::from("00010"));
    }

    #[test]
    fn test_not() {
        let first = Bitmap::from("10101"); // 21

        assert_eq!(
            !&first,
            Bitmap {
                size: first.size,
                chunks: vec![!21]
            }
        );
    }
}
