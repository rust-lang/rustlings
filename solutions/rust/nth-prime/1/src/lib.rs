pub fn nth(n: u32) -> u32 {
    let mut primes= Vec::new();
    let mut canditate = 2;

    while primes.len() <= n as usize  {
        let mut is_prime = true;
            let sqrt_canditate = (canditate as f64).sqrt() as u32;
        for &prime in primes.iter() {
            if prime > sqrt_canditate {
                break;
            }
            if canditate % prime == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            primes.push(canditate);
        }
        if canditate == 2 {
            canditate = 3;
        } else {
            canditate += 2;
        }
    }
    primes[n as usize] 
}
