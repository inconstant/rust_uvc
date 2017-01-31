pub fn largest_prime_factor(n: u64) -> u64 {
    tail_largest_prime_factor(n, 2, 1)
}

fn tail_largest_prime_factor(n: u64, lpf: u64, att: u64) -> u64 {
    if lpf > n {
        return att
    }

    let n_att = if n % lpf == 0 { lpf } else { att };
    let n_lpf = if n % lpf == 0 { lpf } else { lpf + 1 };
    let n_n = if n % lpf == 0 { n/lpf } else { n };

    tail_largest_prime_factor(n_n, n_lpf, n_att)
}
