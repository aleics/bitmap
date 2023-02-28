#![feature(test)]
extern crate test;

use bitmap::*;

const DENSE_BITMAP: &str = "1101011001110101100111010110011101011001110101100111010110011101011001110101100111010110011101011001";
const DENSE_ANOTHER_BITMAP: &str = "10110111011011011101101101110110110111011011011101101101110110110111011011011101101101110110110111011011011101";

fn sparse_bitmap(zeroes: usize, ones: usize) -> String {
    let mut bitmap = String::new();

    for _ in 0..zeroes {
        bitmap.push('0');
    }

    for _ in 0..ones {
        bitmap.push('1');
    }

    bitmap
}

fn dense_bitmap(pattern: &str, times: usize) -> String {
    let mut bitmap = String::new();
    for _ in 0..times {
        bitmap.push_str(pattern);
    }
    bitmap
}

#[cfg(test)]
mod dense_tests {

    use crate::*;
    use test::Bencher;

    fn bitmap() -> String {
        dense_bitmap(DENSE_BITMAP, 1000)
    }

    fn another_bitmap() -> String {
        dense_bitmap(DENSE_ANOTHER_BITMAP, 1000)
    }

    #[bench]
    fn bench_bitmap_get(b: &mut Bencher) {
        let bitmap = Bitmap::from(bitmap().as_str());
        b.iter(|| {
            for i in 0..bitmap.size {
                bitmap.get(i);
            }
        });
    }

    #[bench]
    fn bench_sparse_bitmap_get(b: &mut Bencher) {
        let bitmap = SparseBitmap::from(bitmap().as_str());
        b.iter(|| {
            for i in 0..bitmap.size {
                bitmap.get(i);
            }
        });
    }

    #[bench]
    fn bench_bitmap_set(b: &mut Bencher) {
        let mut bitmap = Bitmap::from(bitmap().as_str());
        b.iter(|| {
            for i in 0..bitmap.size {
                bitmap.set(i, true);
            }
        });
    }

    #[bench]
    fn bench_sparse_bitmap_set(b: &mut Bencher) {
        let mut bitmap = SparseBitmap::from(bitmap().as_str());
        b.iter(|| {
            for i in 0..bitmap.size {
                bitmap.set(i, true);
            }
        });
    }

    #[bench]
    fn bench_bitmap_and(b: &mut Bencher) {
        let first = Bitmap::from(bitmap().as_str());
        let second = Bitmap::from(another_bitmap().as_str());
        b.iter(|| &first & &second);
    }

    #[bench]
    fn bench_sparse_bitmap_and(b: &mut Bencher) {
        let first = SparseBitmap::from(bitmap().as_str());
        let second = SparseBitmap::from(another_bitmap().as_str());
        b.iter(|| &first & &second);
    }

    #[bench]
    fn bench_bitmap_or(b: &mut Bencher) {
        let first = Bitmap::from(bitmap().as_str());
        let second = Bitmap::from(another_bitmap().as_str());
        b.iter(|| &first | &second);
    }

    #[bench]
    fn bench_sparse_bitmap_or(b: &mut Bencher) {
        let first = SparseBitmap::from(bitmap().as_str());
        let second = SparseBitmap::from(another_bitmap().as_str());
        b.iter(|| &first | &second);
    }

    #[bench]
    fn bench_bitmap_not(b: &mut Bencher) {
        let bitmap = Bitmap::from(DENSE_BITMAP);
        b.iter(|| !&bitmap);
    }

    #[bench]
    fn bench_sparse_bitmap_not(b: &mut Bencher) {
        let bitmap = SparseBitmap::from(bitmap().as_str());
        b.iter(|| !&bitmap);
    }

    #[bench]
    fn bench_bitmap_xor(b: &mut Bencher) {
        let first = Bitmap::from(bitmap().as_str());
        let second = Bitmap::from(another_bitmap().as_str());
        b.iter(|| &first ^ &second);
    }

    #[bench]
    fn bench_sparse_bitmap_xor(b: &mut Bencher) {
        let first = SparseBitmap::from(bitmap().as_str());
        let second = SparseBitmap::from(another_bitmap().as_str());
        b.iter(|| &first ^ &second);
    }
}

#[cfg(test)]
mod sparse_tests {

    use crate::*;
    use test::Bencher;

    fn bitmap() -> String {
        sparse_bitmap(100000, 100)
    }

    fn another_bitmap() -> String {
        sparse_bitmap(99900, 200)
    }

    #[bench]
    fn bench_bitmap_get(b: &mut Bencher) {
        let bitmap = Bitmap::from(bitmap().as_str());
        b.iter(|| {
            for i in 0..bitmap.size {
                bitmap.get(i);
            }
        });
    }

    #[bench]
    fn bench_sparse_bitmap_get(b: &mut Bencher) {
        let bitmap = SparseBitmap::from(bitmap().as_str());
        b.iter(|| {
            for i in 0..bitmap.size {
                bitmap.get(i);
            }
        });
    }

    #[bench]
    fn bench_bitmap_set(b: &mut Bencher) {
        let mut bitmap = Bitmap::from(bitmap().as_str());
        b.iter(|| {
            for i in 0..bitmap.size {
                bitmap.set(i, true);
            }
        });
    }

    #[bench]
    fn bench_sparse_bitmap_set(b: &mut Bencher) {
        let mut bitmap = SparseBitmap::from(bitmap().as_str());
        b.iter(|| {
            for i in 0..bitmap.size {
                bitmap.set(i, true);
            }
        });
    }

    #[bench]
    fn bench_bitmap_and(b: &mut Bencher) {
        let first = Bitmap::from(bitmap().as_str());
        let second = Bitmap::from(another_bitmap().as_str());
        b.iter(|| &first & &second);
    }

    #[bench]
    fn bench_sparse_bitmap_and(b: &mut Bencher) {
        let first = SparseBitmap::from(bitmap().as_str());
        let second = SparseBitmap::from(another_bitmap().as_str());
        b.iter(|| &first & &second);
    }

    #[bench]
    fn bench_bitmap_or(b: &mut Bencher) {
        let first = Bitmap::from(bitmap().as_str());
        let second = Bitmap::from(another_bitmap().as_str());
        b.iter(|| &first | &second);
    }

    #[bench]
    fn bench_sparse_bitmap_or(b: &mut Bencher) {
        let first = SparseBitmap::from(bitmap().as_str());
        let second = SparseBitmap::from(another_bitmap().as_str());
        b.iter(|| &first | &second);
    }

    #[bench]
    fn bench_bitmap_not(b: &mut Bencher) {
        let bitmap = Bitmap::from(bitmap().as_str());
        b.iter(|| !&bitmap);
    }

    #[bench]
    fn bench_sparse_bitmap_not(b: &mut Bencher) {
        let bitmap = SparseBitmap::from(bitmap().as_str());
        b.iter(|| !&bitmap);
    }

    #[bench]
    fn bench_bitmap_xor(b: &mut Bencher) {
        let first = Bitmap::from(bitmap().as_str());
        let second = Bitmap::from(another_bitmap().as_str());
        b.iter(|| &first ^ &second);
    }

    #[bench]
    fn bench_sparse_bitmap_xor(b: &mut Bencher) {
        let first = SparseBitmap::from(bitmap().as_str());
        let second = SparseBitmap::from(another_bitmap().as_str());
        b.iter(|| &first ^ &second);
    }
}
