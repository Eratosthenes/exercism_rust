/*
There exists exactly one Pythagorean triplet for which a + b + c = 1000.
Find the product a * b * c.
*/

fn is_square(n: f64) -> bool {
    n.sqrt().floor() == n.sqrt()
}

pub fn find() -> Option<u32> {
    let mut c_sq: f64; // c squared
    let targ: u32 = 1000;

    for a in 1..targ {
        for b in 1..targ {
            c_sq = (a.pow(2) + b.pow(2)) as f64;
            if is_square(c_sq) {
                let c: u32 = c_sq.sqrt() as u32;

                if a + b + c == targ {
                    return Some(a * b * c);
                }
            }
        }
    }
    None
}
