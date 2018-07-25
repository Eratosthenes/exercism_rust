pub fn factors(mut n: u64) -> Vec<u64> {
    let mut prime_factors: Vec<u64> = vec![];
    let mut i = 2; // first prime factor to test

     while n > 1 {
        i = (i..n+1).find(|x| n % x == 0).unwrap(); 
        prime_factors.push(i);
        n /= i;
    } 
    prime_factors
}
