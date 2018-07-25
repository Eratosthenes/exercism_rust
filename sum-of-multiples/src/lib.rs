pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples: Vec<u32> = vec![];
    for factor in factors.iter() {
        multiples.extend(
            (1..)
                .take_while(|x| factor * x < limit)
                .map(|x| factor * x)
        );
    }
    multiples.sort();
    multiples.dedup();
    multiples.iter().sum()
}
