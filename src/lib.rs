#![allow(arithmetic_overflow)]
use ndarray::{Array1, Array2};
use rand::Rng;

pub fn freivald_verify(a: &Array2<u128>, b: &Array2<u128>, c: &Array2<u128>) -> bool {
    unimplemented!()
}

pub fn dumb_verify(a: &Array2<u128>, b: &Array2<u128>, c: &Array2<u128>) -> bool {
    assert!(check_matrix_dimensions(a, b, c));
    a.dot(b) == c
}

pub fn check_matrix_dimensions(a: &Array2<u128>, b: &Array2<u128>, c: &Array2<u128>) -> bool {
    a.nrows() == b.ncols()
        && a.ncols() == b.nrows()
        && c.nrows() == a.nrows()
        && c.ncols() == b.ncols()
}
