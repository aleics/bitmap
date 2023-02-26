#![feature(test)]
extern crate test;

use bitmap::*;

const DENSE_BITMAP: &str = "1101011001110101100111010110011101011001110101100111010110011101011001110101100111010110011101011001";
const DENSE_ANOTHER_BITMAP: &str = "10110111011011011101101101110110110111011011011101101101110110110111011011011101101101110110110111011011011101";

const SPARSE_BITMAP: &str = "0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000011";
const SPARSE_ANOTHER_BITMAP: &str = "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000110";

const TIMES: usize = 1000;

fn create_bitmap(pattern: &str, times: usize) -> Bitmap {
    let mut bitmap = String::new();
    for _ in 0..times {
        bitmap.push_str(pattern);
    }
    Bitmap::from(bitmap.as_str())
}

fn create_sparse_bitmap(pattern: &str, times: usize) -> SparseBitmap {
    let mut bitmap = String::new();
    for _ in 0..times {
        bitmap.push_str(pattern);
    }
    SparseBitmap::from(bitmap.as_str())
}

#[cfg(test)]
mod dense_tests {

    use crate::*;
    use test::Bencher;

    #[bench]
    fn bench_bitmap_get(b: &mut Bencher) {
        let bitmap = create_bitmap(DENSE_BITMAP, TIMES);
        b.iter(|| {
            for i in 0..bitmap.size {
                bitmap.get(i);
            }
        });
    }

    #[bench]
    fn bench_sparse_bitmap_get(b: &mut Bencher) {
        let bitmap = create_bitmap(DENSE_BITMAP, TIMES);
        b.iter(|| {
            for i in 0..bitmap.size {
                bitmap.get(i);
            }
        });
    }

    #[bench]
    fn bench_bitmap_set(b: &mut Bencher) {
        let mut bitmap = create_bitmap(DENSE_BITMAP, TIMES);
        b.iter(|| {
            for i in 0..bitmap.size {
                bitmap.set(i, true);
            }
        });
    }

    #[bench]
    fn bench_sparse_bitmap_set(b: &mut Bencher) {
        let mut bitmap = create_sparse_bitmap(DENSE_BITMAP, TIMES);
        b.iter(|| {
            for i in 0..bitmap.size {
                bitmap.set(i, true);
            }
        });
    }

    #[bench]
    fn bench_bitmap_and(b: &mut Bencher) {
        let first = create_bitmap(DENSE_BITMAP, TIMES);
        let second = create_bitmap(DENSE_ANOTHER_BITMAP, TIMES);
        b.iter(|| &first & &second);
    }

    #[bench]
    fn bench_sparse_bitmap_and(b: &mut Bencher) {
        let first = create_sparse_bitmap(DENSE_BITMAP, TIMES);
        let second = create_sparse_bitmap(DENSE_ANOTHER_BITMAP, TIMES);
        b.iter(|| &first & &second);
    }

    #[bench]
    fn bench_bitmap_or(b: &mut Bencher) {
        let first = create_bitmap(DENSE_BITMAP, TIMES);
        let second = create_bitmap(DENSE_ANOTHER_BITMAP, TIMES);
        b.iter(|| &first | &second);
    }

    #[bench]
    fn bench_sparse_bitmap_or(b: &mut Bencher) {
        let first = create_sparse_bitmap(DENSE_BITMAP, TIMES);
        let second = create_sparse_bitmap(DENSE_ANOTHER_BITMAP, TIMES);
        b.iter(|| &first | &second);
    }

    #[bench]
    fn bench_bitmap_not(b: &mut Bencher) {
        let bitmap = Bitmap::from(DENSE_BITMAP);
        b.iter(|| !&bitmap);
    }

    #[bench]
    fn bench_sparse_bitmap_not(b: &mut Bencher) {
        let bitmap = create_bitmap(DENSE_BITMAP, TIMES);
        b.iter(|| !&bitmap);
    }

    #[bench]
    fn bench_bitmap_xor(b: &mut Bencher) {
        let first = create_bitmap(DENSE_BITMAP, TIMES);
        let second = create_bitmap(DENSE_ANOTHER_BITMAP, TIMES);
        b.iter(|| &first ^ &second);
    }

    #[bench]
    fn bench_sparse_bitmap_xor(b: &mut Bencher) {
        let first = create_sparse_bitmap(DENSE_BITMAP, TIMES);
        let second = create_sparse_bitmap(DENSE_ANOTHER_BITMAP, TIMES);
        b.iter(|| &first ^ &second);
    }
}

#[cfg(test)]
mod sparse_tests {

    use crate::*;
    use test::Bencher;

    const TIMES: usize = 1000;

    #[bench]
    fn bench_bitmap_get(b: &mut Bencher) {
        let bitmap = create_bitmap(SPARSE_BITMAP, TIMES);
        b.iter(|| {
            for i in 0..bitmap.size {
                bitmap.get(i);
            }
        });
    }

    #[bench]
    fn bench_sparse_bitmap_get(b: &mut Bencher) {
        let bitmap = create_bitmap(SPARSE_BITMAP, TIMES);
        b.iter(|| {
            for i in 0..bitmap.size {
                bitmap.get(i);
            }
        });
    }

    #[bench]
    fn bench_bitmap_set(b: &mut Bencher) {
        let mut bitmap = create_bitmap(SPARSE_BITMAP, TIMES);
        b.iter(|| {
            for i in 0..bitmap.size {
                bitmap.set(i, true);
            }
        });
    }

    #[bench]
    fn bench_sparse_bitmap_set(b: &mut Bencher) {
        let mut bitmap = create_sparse_bitmap(SPARSE_BITMAP, TIMES);
        b.iter(|| {
            for i in 0..bitmap.size {
                bitmap.set(i, true);
            }
        });
    }

    #[bench]
    fn bench_bitmap_and(b: &mut Bencher) {
        let first = create_bitmap(SPARSE_BITMAP, TIMES);
        let second = create_bitmap(SPARSE_ANOTHER_BITMAP, TIMES);
        b.iter(|| &first & &second);
    }

    #[bench]
    fn bench_sparse_bitmap_and(b: &mut Bencher) {
        let first = create_sparse_bitmap(SPARSE_BITMAP, TIMES);
        let second = create_sparse_bitmap(SPARSE_ANOTHER_BITMAP, TIMES);
        b.iter(|| &first & &second);
    }

    #[bench]
    fn bench_bitmap_or(b: &mut Bencher) {
        let first = create_bitmap(SPARSE_BITMAP, TIMES);
        let second = create_bitmap(SPARSE_ANOTHER_BITMAP, TIMES);
        b.iter(|| &first | &second);
    }

    #[bench]
    fn bench_sparse_bitmap_or(b: &mut Bencher) {
        let first = create_sparse_bitmap(SPARSE_BITMAP, TIMES);
        let second = create_sparse_bitmap(SPARSE_ANOTHER_BITMAP, TIMES);
        b.iter(|| &first | &second);
    }

    #[bench]
    fn bench_bitmap_not(b: &mut Bencher) {
        let bitmap = create_bitmap(SPARSE_BITMAP, TIMES);
        b.iter(|| !&bitmap);
    }

    #[bench]
    fn bench_sparse_bitmap_not(b: &mut Bencher) {
        let bitmap = create_sparse_bitmap(SPARSE_BITMAP, TIMES);
        b.iter(|| !&bitmap);
    }

    #[bench]
    fn bench_bitmap_xor(b: &mut Bencher) {
        let first = create_bitmap(SPARSE_BITMAP, TIMES);
        let second = create_bitmap(SPARSE_ANOTHER_BITMAP, TIMES);
        b.iter(|| &first ^ &second);
    }

    #[bench]
    fn bench_sparse_bitmap_xor(b: &mut Bencher) {
        let first = create_sparse_bitmap(SPARSE_BITMAP, TIMES);
        let second = create_sparse_bitmap(SPARSE_ANOTHER_BITMAP, TIMES);
        b.iter(|| &first ^ &second);
    }
}
