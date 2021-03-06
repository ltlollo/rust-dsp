extern crate dsp;
extern crate num;

use num::complex::Complex;
use dsp::*;

#[test]
fn test_dfts() {
    let (four, one, zero) = (Complex::new(4.0f64, 0.0),
                             Complex::new(1.0f64, 0.0),
                             Complex::new(0.0f64, 0.0));
    let result = vec![four, zero, zero, zero, four, zero, zero, zero];
    let sig_orig = vec![one, zero, one, zero, one, zero, one, zero];
    let mut sig = sig_orig.clone();
    dif(&mut sig[..]);
    assert!(sig == result, "testing dif");
    sig = sig_orig.clone();
    dit(&mut sig[..]);
    assert!(sig == result, "testing dit");
}

#[test]
fn test_dfts_table() {
    let (four, one, zero) = (Complex::new(4.0f64, 0.0),
                             Complex::new(1.0f64, 0.0),
                             Complex::new(0.0f64, 0.0));
    let mut table = TwiddleTable::new();
    let result = vec![four, zero, zero, zero, four, zero, zero, zero];
    let sig_orig = vec![one, zero, one, zero, one, zero, one, zero];
    let mut sig = sig_orig.clone();
    table.dif(&mut sig[..]);
    assert!(sig == result, "testing dif");
    sig = sig_orig.clone();
    table.dit(&mut sig[..]);
    assert!(sig == result, "testing dit");
}

#[test]
fn test_fhwts() {
    let mut sig = vec![4f64, 2f64, 2f64, 4f64];
    let res = vec![3f64, 1f64, 0f64, -1f64];
    let orig = sig.clone();
    fhwt(&mut sig[..]).unwrap();
    assert!(sig == res, "testing fhwt");
    fihwt(&mut sig[..]).unwrap();
    assert!(sig == orig, "testing fihwt");
}

#[test]
fn test_wrong_size_fhwt() {
    let mut sig = vec![4f64; 3];
    assert!(fhwt(&mut sig[..]) == None, "testing fhwt for non pow2 sizes");
}

#[test]
fn test_wrong_size_fihwt() {
    let mut sig = vec![4f64; 3];
    assert!(fihwt(&mut sig[..]) == None, "testing fihwt for non pow2 sizes");
}
