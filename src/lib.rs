use std::ops::{BitAnd, BitOr, BitXor, Not};

/// Bitmap stores a bitmap in chunks of 64 bits
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Bitmap {
    chunks: Vec<usize>,
    pub size: usize,
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
    #[inline(always)]
    pub fn get(&self, position: usize) -> bool {
        let (chunk_index, bit_index_in_chunk) = bit_index(position, usize::BITS as usize);
        let chunk = self.chunks[chunk_index];

        // position_bit is a 1 in the bit position of the desired index
        let position_bit = 1 << bit_index_in_chunk;

        // Using AND with `position_bit` returns only the value at the desired position
        // If it's equal to 0, it means bit at the `position` was 0. Otherwise, 1.
        (chunk & position_bit) != 0
    }

    /// Set a bit value in a given position
    #[inline(always)]
    pub fn set(&mut self, position: usize, value: bool) {
        if position >= self.size {
            panic!("Index out of bounds");
        }

        let (chunk_index, bit_index_in_chunk) = bit_index(position, usize::BITS as usize);

        if value {
            self.set_one(chunk_index, bit_index_in_chunk)
        } else {
            self.set_zero(chunk_index, bit_index_in_chunk)
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
    #[inline(always)]
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
    #[inline(always)]
    fn set_zero(&mut self, chunk: usize, bit: usize) {
        self.chunks[chunk] &= !(1 << bit)
    }

    fn chunks_with_size(size: usize) -> Vec<usize> {
        vec![0; chunks_count(size, usize::BITS as usize)]
    }
}

impl BitAnd for &Bitmap {
    type Output = Bitmap;

    #[inline(always)]
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

    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        let size = self.size.min(rhs.size);
        let mut chunks = Bitmap::chunks_with_size(size);

        for (id, chunk) in chunks.iter_mut().enumerate() {
            *chunk = self.chunks[id] | rhs.chunks[id];
        }

        Bitmap { chunks, size }
    }
}

impl BitXor for &Bitmap {
    type Output = Bitmap;

    #[inline(always)]
    fn bitxor(self, rhs: Self) -> Self::Output {
        let size = self.size.min(rhs.size);
        let mut chunks = Bitmap::chunks_with_size(size);

        for (id, chunk) in chunks.iter_mut().enumerate() {
            *chunk = self.chunks[id] ^ rhs.chunks[id];
        }

        Bitmap { chunks, size }
    }
}

impl Not for &Bitmap {
    type Output = Bitmap;

    #[inline(always)]
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
                '1' => bitmap.set(index, true),
                '0' => bitmap.set(index, false),
                _ => panic!("Can not convert from string slice. Unexpected character {char}"),
            }
        }

        bitmap
    }
}

// SparseBitmap is a bitmap representation optimized for sparse bitmap distributions.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SparseBitmap {
    runs: Vec<Run>,
    pub size: usize,
}

impl SparseBitmap {
    /// Creates a new `SparseBitmap` with a fixed size
    pub fn new(size: usize) -> SparseBitmap {
        SparseBitmap {
            runs: Vec::new(),
            size,
        }
    }

    /// Get the bit value from a given position
    #[inline(always)]
    pub fn get(&self, position: usize) -> bool {
        if position > self.size {
            return false;
        }

        for run in &self.runs {
            if run.start <= position && run.end() > position {
                return true;
            }
        }

        false
    }

    /// Set a bit value in a given position
    #[inline(always)]
    pub fn set(&mut self, position: usize, value: bool) {
        if position >= self.size {
            panic!("Index out of bounds");
        }

        // Check if the position collides with existing runs
        if let Some(index) = self
            .runs
            .iter()
            .position(|run| run.start <= position && run.end() >= position)
        {
            match value {
                true => self.set_one(position, index),
                false => self.set_zero(position, index),
            }
        } else if value {
            self.append(Run::new(position, 1));
        }
    }

    /// Set bit value to 1 for a given position in-between a run with the given index.
    #[inline(always)]
    fn set_one(&mut self, position: usize, index: usize) {
        let current = self.runs.get(index).unwrap();

        if position == current.start {
            // Find the new start by merging any conflicts

            let start = if current.start == 0 {
                current.start
            } else if let Some(index) = self
                .runs
                .iter()
                .position(|run| run.end() == current.start - 1)
            {
                self.runs.remove(index).start
            } else {
                current.start - 1
            };

            let run = self.runs.get_mut(index).unwrap();
            run.start = start
        } else if position == current.end() {
            // Find the new length by merging any conflicts
            let length = if let Some(index) = self
                .runs
                .iter()
                .position(|run| run.start == current.end() + 1)
            {
                self.runs.remove(index).length + 1
            } else {
                1
            };

            let run = self.runs.get_mut(index).unwrap();
            run.length += length;
        }
    }

    /// Set bit value to 0 for a given position in-between a run with the given index.
    #[inline(always)]
    fn set_zero(&mut self, position: usize, index: usize) {
        let run = self.runs.get_mut(index).unwrap();

        if position == run.start {
            run.start += 1
        } else if position == run.end() {
            run.length -= 1
        } else {
            // If a 0 is set in-between a run, create a new run with the leftover
            let start = position + 1;
            let end = run.end();

            run.length = position - run.start;
            if start < end - 1 {
                self.runs.push(Run::new(start, end - start));
            }
        }
    }

    #[inline(always)]
    fn append(&mut self, run: Run) {
        if let Some((last, union)) = self
            .runs
            .last_mut()
            .and_then(|last| last.union(&run).map(|union| (last, union)))
        {
            *last = union;
        } else {
            self.runs.push(run);
        }
    }
}

impl BitAnd for &SparseBitmap {
    type Output = SparseBitmap;

    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        let size = self.size.min(rhs.size);
        let mut runs = Vec::new();

        let mut iter = self.runs.iter();
        let mut rhs_iter = rhs.runs.iter();

        let mut next = iter.next();
        let mut rhs_next = rhs_iter.next();

        while let (Some(run), Some(rhs_run)) = (next, rhs_next) {
            if let Some(intersect) = run.intersect(rhs_run) {
                runs.push(intersect);
            }

            // Iterate to the next run by increasing the pointer of the
            // run with smallest end, so that we cover that a single long
            // run has multiple intersections.
            if run.end() < rhs_run.end() {
                next = iter.next();
            } else {
                rhs_next = rhs_iter.next()
            }
        }

        SparseBitmap { runs, size }
    }
}

impl BitOr for &SparseBitmap {
    type Output = SparseBitmap;

    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        let size = self.size.min(rhs.size);
        let mut sparse = SparseBitmap::new(size);

        let mut iter = self.runs.iter();
        let mut rhs_iter = rhs.runs.iter();

        let mut next = iter.next();
        let mut rhs_next = rhs_iter.next();

        // Iterate through both bitmaps and append all runs together
        loop {
            match (next, rhs_next) {
                (Some(run), Some(rhs_run)) => {
                    sparse.append(*run);
                    sparse.append(*rhs_run);
                    next = iter.next();
                    rhs_next = rhs_iter.next();
                }
                (Some(run), None) => {
                    sparse.append(*run);
                    next = iter.next();
                }
                (None, Some(run)) => {
                    sparse.append(*run);
                    rhs_next = rhs_iter.next();
                }
                (None, None) => break,
            }
        }

        sparse
    }
}

impl Not for &SparseBitmap {
    type Output = SparseBitmap;

    #[inline(always)]
    fn not(self) -> Self::Output {
        let mut start: usize = 0;
        let mut runs = Vec::new();

        for run in &self.runs {
            if start < run.start {
                runs.push(Run::new(start, run.start - start));
            }

            start = run.end();
        }

        SparseBitmap {
            size: self.size,
            runs,
        }
    }
}

impl BitXor for &SparseBitmap {
    type Output = SparseBitmap;

    #[inline(always)]
    fn bitxor(self, rhs: Self) -> Self::Output {
        let size = self.size.min(rhs.size);
        let mut bitmap = SparseBitmap::new(size);

        let mut iter = self.runs.iter();
        let mut rhs_iter = rhs.runs.iter();

        let mut next = iter.next();
        let mut rhs_next = rhs_iter.next();

        let mut previous_end = None;

        loop {
            match (next, rhs_next) {
                // If both there are runs for both bitmaps
                (Some(run), Some(rhs_run)) => {
                    let start = run.start;
                    let end = run.end();

                    let rhs_start = rhs_run.start;
                    let rhs_end = rhs_run.end();

                    // New start is the minimum of both runs start or the previous run end point,
                    // in case it exists and is between both run start points.
                    let mut new_start = run.start.min(rhs_run.start);

                    if let Some(previous_end) = previous_end {
                        // If previous end point is between the both runs start points.
                        if (previous_end > start && previous_end < rhs_start)
                            || (previous_end > rhs_start && previous_end < start)
                        {
                            new_start = previous_end;
                        }
                    }

                    // New end is the maximum start point or the minimum end of current runs.
                    let new_end = start.max(rhs_start).min(end).min(rhs_end);

                    let length = new_end - new_start;

                    if length > 0 {
                        bitmap.append(Run::new(new_start, length));
                    }

                    // Iterate to the next run by increasing the pointer of the
                    // run with smallest end, so that we cover that a single long
                    // run has multiple intersections.
                    if end < rhs_end {
                        next = iter.next();
                        previous_end = Some(end);
                    } else {
                        rhs_next = rhs_iter.next();
                        previous_end = Some(rhs_end);
                    }
                }
                // If only left bitmap has runs left
                (Some(run), None) => {
                    let end = run.end();

                    // If previous end exists and it's smaller than current end.
                    if let Some(previous_end) = previous_end {
                        if previous_end < end {
                            bitmap.append(Run::new(previous_end, end - previous_end));
                        }
                    } else {
                        bitmap.append(*run);
                    }

                    next = iter.next();
                    previous_end = Some(end);
                }
                // If only right bitmap has runs left
                (None, Some(rhs_run)) => {
                    let rhs_end = rhs_run.end();

                    // If previous end exists and it's smaller than current end.
                    if let Some(previous_end) = previous_end {
                        if previous_end < rhs_end {
                            bitmap.append(Run::new(previous_end, rhs_end - previous_end));
                        }
                    } else {
                        bitmap.append(*rhs_run);
                    }

                    rhs_next = rhs_iter.next();
                    previous_end = Some(rhs_end);
                }
                (None, None) => break,
            }
        }

        bitmap
    }
}

impl From<&str> for SparseBitmap {
    fn from(value: &str) -> Self {
        let size = value.len();
        let mut runs = Vec::new();

        let mut start_run = None;

        for (index, char) in value.chars().rev().enumerate() {
            if char == '1' {
                if start_run.is_none() {
                    start_run = Some(index);
                }
            } else if char == '0' {
                if let Some(start) = start_run {
                    runs.push(Run::new(start, index - start));
                    start_run = None;
                }
            }
        }

        if let Some(start) = start_run {
            runs.push(Run::new(start, size - start));
        }

        SparseBitmap { runs, size }
    }
}

impl ToString for SparseBitmap {
    fn to_string(&self) -> String {
        let mut result = (0..self.size).fold(String::with_capacity(self.size), |mut acc, _| {
            acc.push('0');
            acc
        });

        for run in &self.runs {
            let ones = (0..run.length).fold(String::with_capacity(run.length), |mut acc, _| {
                acc.push('1');
                acc
            });

            let start = self.size - (run.start + run.length);
            let end = self.size - run.start;
            result.replace_range(start..end, &ones)
        }

        result
    }
}

// Run represents a range in a `SparseBitmap`, where 1s are stored
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Run {
    start: usize,
    length: usize,
}

impl Run {
    fn new(start: usize, length: usize) -> Run {
        Run { start, length }
    }

    #[inline(always)]
    fn end(&self) -> usize {
        self.start + self.length
    }

    #[inline(always)]
    fn intersect(&self, run: &Run) -> Option<Run> {
        if self.matches(run) {
            let start = self.start.max(run.start);
            let end = self.end().min(run.end());

            Some(Run::new(start, end - start))
        } else {
            None
        }
    }

    #[inline(always)]
    fn union(&self, run: &Run) -> Option<Run> {
        if self.matches(run) {
            let start = self.start.min(run.start);
            let end = self.end().max(run.end());

            Some(Run::new(start, end - start))
        } else {
            None
        }
    }

    #[inline(always)]
    fn matches(&self, run: &Run) -> bool {
        self.contains(run.start) || run.contains(self.start)
    }

    #[inline(always)]
    fn contains(&self, index: usize) -> bool {
        index >= self.start && index <= self.end()
    }
}

/// Calculate the amount of chunks needed for the desired bitmap size, and the bits per chunk.
#[inline(always)]
fn chunks_count(size: usize, chunk_bit_size: usize) -> usize {
    (size + chunk_bit_size - 1) / chunk_bit_size
}

/// Calculate the bit index in the chunks by a given position, and chunk bit size.
#[inline(always)]
fn bit_index(position: usize, chunk_bit_size: usize) -> (usize, usize) {
    let chunk_index = position / chunk_bit_size;
    let bit_index_in_chunk = position % chunk_bit_size;

    (chunk_index, bit_index_in_chunk)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitmap_chunks_of_size() {
        assert_eq!(0, chunks_count(0, 16));
        assert_eq!(1, chunks_count(1, 16));
        assert_eq!(1, chunks_count(15, 16));
        assert_eq!(1, chunks_count(16, 16));
        assert_eq!(2, chunks_count(17, 16));
    }

    #[test]
    fn test_bitmap_get() {
        let bitmap = Bitmap::from("11001");

        assert_eq!(bitmap.get(0), true);
        assert_eq!(bitmap.get(1), false);
        assert_eq!(bitmap.get(2), false);
        assert_eq!(bitmap.get(3), true);
        assert_eq!(bitmap.get(4), true);
        assert_eq!(bitmap.get(5), false);
        assert_eq!(bitmap.get(6), false);
    }

    #[test]
    fn test_bitmap_set() {
        let mut bitmap = Bitmap::from("00111");
        bitmap.set(4, true);
        assert_eq!(bitmap, Bitmap::from("10111"));
        bitmap.set(4, false);
        assert_eq!(bitmap, Bitmap::from("00111"));
    }

    #[test]
    fn test_bitmap_or() {
        let first = Bitmap::from("00001");
        let second = Bitmap::from("00010");

        assert_eq!(&first | &second, Bitmap::from("00011"));
    }

    #[test]
    fn test_bitmap_and() {
        let first = Bitmap::from("00011");
        let second = Bitmap::from("00010");

        assert_eq!(&first & &second, Bitmap::from("00010"));
    }

    #[test]
    fn test_bitmap_not() {
        let first = Bitmap::from("10101"); // 21

        assert_eq!(
            !&first,
            Bitmap {
                size: first.size,
                chunks: vec![!21]
            }
        );
    }

    #[test]
    fn test_bitmap_xor() {
        assert_eq!(
            &Bitmap::from("00011") ^ &Bitmap::from("00010"),
            Bitmap::from("00001")
        );
        assert_eq!(
            &Bitmap::from("11001") ^ &Bitmap::from("01100"),
            Bitmap::from("10101")
        );
        assert_eq!(
            &Bitmap::from("11111") ^ &Bitmap::from("01101"),
            Bitmap::from("10010")
        );
        assert_eq!(
            &Bitmap::from("11111") ^ &Bitmap::from("11111"),
            Bitmap::from("00000")
        );
        assert_eq!(
            &Bitmap::from("00110") ^ &Bitmap::from("00110"),
            Bitmap::from("00000")
        );
        assert_eq!(
            &Bitmap::from("10101") ^ &Bitmap::from("01010"),
            Bitmap::from("11111")
        );
        assert_eq!(
            &Bitmap::from("11111") ^ &Bitmap::from("00000"),
            Bitmap::from("11111")
        );
    }

    #[test]
    fn test_set_sparse() {
        let mut bitmap = SparseBitmap::new(5);

        bitmap.set(0, true);
        bitmap.set(1, true);
        bitmap.set(2, true);

        assert_eq!(bitmap, SparseBitmap::from("00111"));

        bitmap = SparseBitmap::from("11111");

        bitmap.set(0, true);
        bitmap.set(1, true);
        bitmap.set(2, true);

        assert_eq!(bitmap, SparseBitmap::from("11111"));

        bitmap.set(0, false);
        bitmap.set(2, false);
        bitmap.set(4, false);

        assert_eq!(bitmap, SparseBitmap::from("01010"));
    }

    #[test]
    fn test_set_merges_runs_sparse() {
        let mut bitmap = SparseBitmap::new(5);

        bitmap.set(0, true);
        bitmap.set(2, true);
        bitmap.set(3, true);
        bitmap.set(1, true);

        assert_eq!(bitmap.runs, vec![Run::new(0, 4)]);
        assert_eq!(bitmap, SparseBitmap::from("01111"));
    }

    #[test]
    fn test_set_splits_runs_sparse() {
        let mut bitmap = SparseBitmap::new(5);

        bitmap.set(0, true);
        bitmap.set(1, true);
        bitmap.set(2, true);
        bitmap.set(3, true);
        bitmap.set(4, true);
        bitmap.set(1, true);

        bitmap.set(2, false);

        assert_eq!(bitmap.runs, vec![Run::new(0, 2), Run::new(3, 2)]);
        assert_eq!(bitmap, SparseBitmap::from("11011"));
    }

    #[test]
    fn test_set_add_zero_empty_runs_sparse() {
        let mut bitmap = SparseBitmap::new(5);

        bitmap.set(0, false);
        bitmap.set(1, false);
        bitmap.set(2, false);
        bitmap.set(3, false);

        assert_eq!(bitmap.runs, vec![]);
        assert_eq!(bitmap, SparseBitmap::from("00000"));
    }

    #[test]
    fn test_get_sparse() {
        let bitmap = SparseBitmap::from("11001");

        assert_eq!(bitmap.get(0), true);
        assert_eq!(bitmap.get(1), false);
        assert_eq!(bitmap.get(2), false);
        assert_eq!(bitmap.get(3), true);
        assert_eq!(bitmap.get(4), true);
        assert_eq!(bitmap.get(5), false);
        assert_eq!(bitmap.get(6), false);
    }

    #[test]
    fn test_intersect_sparse_runs() {
        assert_eq!(
            Run::new(2, 4).intersect(&Run::new(1, 3)),
            Some(Run::new(2, 2))
        );
        assert_eq!(
            Run::new(3, 2).intersect(&Run::new(1, 4)),
            Some(Run::new(3, 2))
        );
        assert_eq!(
            Run::new(2, 2).intersect(&Run::new(3, 2)),
            Some(Run::new(3, 1))
        );
        assert_eq!(Run::new(2, 1).intersect(&Run::new(4, 1)), None);
        assert_eq!(Run::new(4, 1).intersect(&Run::new(2, 1)), None);
    }

    #[test]
    fn test_union_sparse_runs() {
        assert_eq!(Run::new(2, 4).union(&Run::new(1, 3)), Some(Run::new(1, 5)));
        assert_eq!(Run::new(3, 2).union(&Run::new(1, 4)), Some(Run::new(1, 4)));
        assert_eq!(Run::new(2, 2).union(&Run::new(3, 2)), Some(Run::new(2, 3)));
        assert_eq!(Run::new(2, 1).union(&Run::new(4, 1)), None);
        assert_eq!(Run::new(4, 1).union(&Run::new(2, 1)), None);
    }

    #[test]
    fn test_sparse_and() {
        let first = SparseBitmap::from("00011");
        let second = SparseBitmap::from("01010");

        assert_eq!(&first & &second, SparseBitmap::from("00010"));
    }

    #[test]
    fn test_sparse_or() {
        let first = SparseBitmap::from("01001");
        let second = SparseBitmap::from("10010");

        assert_eq!(&first | &second, SparseBitmap::from("11011"));
    }

    #[test]
    fn test_sparse_not() {
        assert_eq!(!&SparseBitmap::from("10101"), SparseBitmap::from("01010"));
        assert_eq!(!&SparseBitmap::from("11000"), SparseBitmap::from("00111"));
        assert_eq!(!&SparseBitmap::from("11011"), SparseBitmap::from("00100"));
        assert_eq!(!&SparseBitmap::from("11111"), SparseBitmap::from("00000"));
    }

    #[test]
    fn test_sparse_xor() {
        assert_eq!(
            &SparseBitmap::from("00011") ^ &SparseBitmap::from("00010"),
            SparseBitmap::from("00001")
        );
        assert_eq!(
            &SparseBitmap::from("11001") ^ &SparseBitmap::from("01100"),
            SparseBitmap::from("10101")
        );
        assert_eq!(
            &SparseBitmap::from("11111") ^ &SparseBitmap::from("01101"),
            SparseBitmap::from("10010")
        );
        assert_eq!(
            &SparseBitmap::from("11111") ^ &SparseBitmap::from("11111"),
            SparseBitmap::from("00000")
        );
        assert_eq!(
            &SparseBitmap::from("00110") ^ &SparseBitmap::from("00110"),
            SparseBitmap::from("00000")
        );
        assert_eq!(
            &SparseBitmap::from("10101") ^ &SparseBitmap::from("01010"),
            SparseBitmap::from("11111")
        );
        assert_eq!(
            &SparseBitmap::from("11111") ^ &SparseBitmap::from("00000"),
            SparseBitmap::from("11111")
        );
    }
}
