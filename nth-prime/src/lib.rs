fn is_prime(curr_n: u32, primes: &Vec<u32>) -> bool {
    let sqr = (curr_n as f64).sqrt().ceil() as u32;
    let mut prime_range = primes.iter().take_while(|x| **x <= sqr);
    prime_range.all(|p| curr_n % p != 0)
}

pub fn nth(n: u32) -> Option<u32> {
    if n == 0 {
        return None;
    }
    
    let (mut last_prime, mut curr_n) = (2, 3);
    let mut primes: Vec<u32> = vec![last_prime];
    while (primes.len() as u32) < n {
        if is_prime(curr_n, &primes) {
            last_prime = curr_n;
            primes.push(last_prime);
        }
        curr_n += 2;
    }

    Some(last_prime)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_nth() {
        assert_eq!(Some(2), nth(1));
        assert_eq!(Some(3), nth(2));
        assert_eq!(Some(5), nth(3));
        assert_eq!(Some(7), nth(4));
        assert_eq!(Some(11), nth(5));
    }
    
    #[test]
    fn test_big() {
        assert_eq!(Some(104743), nth(10001));
    }
}
