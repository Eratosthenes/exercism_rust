extern crate nth_prime as np;

#[test]
#[ignore]
fn test_first_prime() {
    assert_eq!(np::nth(1), Some(2));
}

#[test]
#[ignore]
fn test_second_prime() {
    assert_eq!(np::nth(2), Some(3));
}

#[test]
#[ignore]
fn test_sixth_prime() {
    assert_eq!(np::nth(6), Some(13));
}

#[test]
#[ignore]
fn test_big_prime() {
    assert_eq!(np::nth(10001), Some(104743));
}

#[test]
#[ignore]
fn test_zeroth_prime() {
    assert_eq!(np::nth(0), None);
}
