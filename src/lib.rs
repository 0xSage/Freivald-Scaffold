use ark_bls12_381::Fr;
use ndarray::Array2;

pub fn freivald_verify(a: &Array2<Fr>, b: &Array2<Fr>, c: &Array2<Fr>) -> bool {
    unimplemented!()
}

pub fn dumb_verify(a: &Array2<Fr>, b: &Array2<Fr>, c: &Array2<Fr>) -> bool {
    assert!(check_matrix_dimensions(a, b, c));
    a.dot(b) == c
}

pub fn check_matrix_dimensions(a: &Array2<Fr>, b: &Array2<Fr>, c: &Array2<Fr>) -> bool {
    a.nrows() == b.ncols()
        && a.ncols() == b.nrows()
        && c.nrows() == a.nrows()
        && c.ncols() == b.ncols()
}
